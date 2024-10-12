extern crate image;
use image::{ImageBuffer, Rgb, RgbImage};
use palette::{IntoColor, Oklab, Srgb};
use rayon::prelude::*;

use crate::utils;

/// Convert a 3 channel histogram into a 3 channel cumulative distribution function.
fn histograms_to_cdfs(histograms: &[usize; 256 * 3]) -> [usize; 256 * 3] {
    // Create a vector to hold the cumulative distribution function
    let cdfs: Vec<usize> = (0..3)
        .flat_map(|channel| {
            let mut cumulative = 0;
            (0..256).map(move |i| {
                cumulative += histograms[channel * 256 + i];
                cumulative
            })
        })
        .collect();

    // Convert the vector to an array
    cdfs.try_into().expect("Expected to convert Vec to array")
}

/// Match the mean and standard deviation of two images.
fn match_palette_img(
    img: &RgbImage,
    src_mean: &[f32; 3],
    tgt_mean: &[f32; 3],
    src_std: &[f32; 3],
    tgt_std: &[f32; 3],
) -> RgbImage {
    let mut matched_image = ImageBuffer::new(img.width(), img.height());
    for (pixel, matched_pixel) in img.pixels().zip(matched_image.pixels_mut()) {
        let matched = pixel
            .0
            .iter()
            .enumerate()
            .map(|(i, &ch)| {
                let scaled = if src_std[i] > 0.0 {
                    ((ch as f32 - src_mean[i]) / src_std[i]) * tgt_std[i] + tgt_mean[i]
                } else {
                    // If source std is zero, just take the target mean
                    tgt_mean[i]
                };
                scaled.clamp(0.0, 255.0).round() as u8
            })
            .collect::<Vec<u8>>();
        *matched_pixel = Rgb(matched
            .try_into()
            .expect("Expected to convert Vec to array"));
    }
    matched_image
}

/// Equalize the color distribution of an image.
fn equalize_img(img: &RgbImage, cdfs: &[usize; 256 * 3], total_pixels: &usize) -> RgbImage {
    let mut equalized = img.clone();
    for channel in 0..3 {
        // Normalize CDF
        let cdf_min = cdfs.iter().find(|&&x| x > 0).cloned().unwrap_or(0);
        for pixel in equalized.pixels_mut() {
            pixel[channel] = (((cdfs[pixel[channel] as usize] - cdf_min) as f32
                / (total_pixels - cdf_min) as f32)
                * 255.0) as u8;
        }
    }
    equalized
}

trait ToLab {
    fn to_lab(&self) -> Self;
}
trait ToSrgb {
    fn to_srgb(&self) -> Self;
}

// the ToLab and ToSrgb traits shouldn't really be used by the user because the match methods
// expect to start in srgb. So they are kept private.
#[allow(private_bounds)]
pub trait ColorMatch: ToLab + ToSrgb {
    /// Compute the mean color.
    fn mean(&self) -> [f32; 3];
    /// Compute the stadndard deviation of the color channel distributions.
    fn std(&self, mean: &[f32; 3]) -> [f32; 3];
    /// Transfer the color palette of the `other` onto `self`, using the Reinhard color transfer algorithm.
    ///
    /// See:
    ///     https://api.semanticscholar.org/CorpusID:14088925
    ///
    /// It matches the mean and standard deviations in the [`Oklab`] color space, then projects
    /// back to [`Srgb`].
    fn match_palette(&self, other: &impl ColorMatch) -> Self;
    /// Equalize the color distribution.
    ///
    /// See:
    ///     https://docs.opencv.org/4.x/d5/daf/tutorial_py_histogram_equalization.html
    ///
    /// It spreads out the distribution such that they cover the full color space.
    fn equalize(&self) -> Self;
}
impl ToSrgb for RgbImage {
    fn to_srgb(&self) -> RgbImage {
        // Create an iterator over the LAB pixels and convert them back to RGB
        let rgb_pixels: Vec<Rgb<u8>> = self
            .pixels()
            .map(|pixel| {
                // Convert LAB to linear RGB
                let lab = Oklab::new(
                    pixel[0] as f32 / 255.,
                    pixel[1] as f32 / 255. - 0.5,
                    pixel[2] as f32 / 255. - 0.5,
                );
                let srgb: Srgb = Srgb::from_linear(lab.into_color());
                Rgb([
                    (srgb.red * 255.).round() as u8,
                    (srgb.green * 255.).round() as u8,
                    (srgb.blue * 255.).round() as u8,
                ])
            })
            .collect();

        // Create a new ImageBuffer from the RGB pixel data
        ImageBuffer::from_fn(self.width(), self.height(), |x, y| {
            rgb_pixels[(y * self.width() + x) as usize]
        })
    }
}

impl ToLab for RgbImage {
    fn to_lab(&self) -> RgbImage {
        // Create an iterator over the pixels and convert them to LAB
        let lab_pixels: Vec<Rgb<u8>> = self
            .pixels()
            .map(|pixel| {
                // Convert from RGB<u8> to linear RGB
                let lab: Oklab = Srgb::from([pixel[0], pixel[1], pixel[2]])
                    .into_linear()
                    .into_color();
                // Return LAB as Rgb<f32>
                // L [0, 1]
                // a [-0.5, 0.5]
                // b [-0.5, 0.5]
                // Should map to [0, 255]
                Rgb([
                    (lab.l * 255.).round().clamp(0., 255.) as u8,
                    (lab.a * 255. + 127.5).round().clamp(0., 255.) as u8,
                    (lab.b * 255. + 127.5).round().clamp(0., 255.) as u8,
                ])
            })
            .collect();

        // Create a new ImageBuffer from the LAB pixel data
        ImageBuffer::from_fn(self.width(), self.height(), |x, y| {
            lab_pixels[(y * self.width() + x) as usize] // Access the pixel data
        })
    }
}

impl ColorMatch for RgbImage {
    fn mean(&self) -> [f32; 3] {
        let mut sum = [0.0; 3];
        let total_pixels = (self.width() * self.height()) as usize;

        for pixel in self.pixels() {
            for (i, &ch) in pixel.0.iter().enumerate() {
                sum[i] += ch as f32;
            }
        }

        sum.iter_mut().for_each(|s| *s /= total_pixels as f32);
        sum
    }

    fn std(&self, mean: &[f32; 3]) -> [f32; 3] {
        let mut sum_squared_diff = [0.0; 3];
        let total_pixels = (self.width() * self.height()) as usize;

        for pixel in self.pixels() {
            for (i, &ch) in pixel.0.iter().enumerate() {
                sum_squared_diff[i] += (ch as f32 - mean[i]).powi(2);
            }
        }

        sum_squared_diff
            .iter_mut()
            .for_each(|s| *s = (*s / total_pixels as f32).sqrt());
        sum_squared_diff
    }

    fn match_palette(&self, other: &impl ColorMatch) -> RgbImage {
        let self_lab = self.to_lab();
        let other_lab = other.to_lab();

        let src_mean = self_lab.mean();
        let src_std = self_lab.std(&src_mean);
        let tgt_mean = other_lab.mean();
        let tgt_std = other_lab.std(&tgt_mean);

        match_palette_img(&self_lab, &src_mean, &tgt_mean, &src_std, &tgt_std).to_srgb()
    }

    fn equalize(&self) -> RgbImage {
        // Three channels: R, G, B
        let mut histograms = [0; 256 * 3];
        for pixel in self.pixels() {
            for (i, &ch) in pixel.0.iter().enumerate() {
                // Increment the histogram
                histograms[i * 256 + ch as usize] += 1;
            }
        }
        // Three channels: R, G, B
        let total_pixels = (self.width() * self.height()) as usize;
        let cdfs = histograms_to_cdfs(&histograms);

        equalize_img(self, &cdfs, &total_pixels)
    }
}

impl ToSrgb for Vec<RgbImage> {
    fn to_srgb(&self) -> Vec<RgbImage> {
        utils::iter_or_par_iter!(self)
            .map(|img| img.to_srgb())
            .collect::<Vec<_>>()
    }
}
impl ToLab for Vec<RgbImage> {
    fn to_lab(&self) -> Vec<RgbImage> {
        utils::iter_or_par_iter!(self)
            .map(|img| img.to_lab())
            .collect::<Vec<_>>()
    }
}
// Implement ColorMatch for a slice of RgbImage
impl ColorMatch for Vec<RgbImage> {
    fn mean(&self) -> [f32; 3] {
        let mut sum = [0.0; 3];
        let mut total_pixels = 0;

        for image in self.iter() {
            for pixel in image.pixels() {
                for (i, &ch) in pixel.0.iter().enumerate() {
                    sum[i] += ch as f32;
                }
            }
            total_pixels += (image.width() * image.height()) as usize;
        }

        sum.iter_mut().for_each(|s| *s /= total_pixels as f32);
        sum
    }

    fn std(&self, mean: &[f32; 3]) -> [f32; 3] {
        let mut sum_squared_diff = [0.0; 3];
        let mut total_pixels = 0;

        for image in self.iter() {
            for pixel in image.pixels() {
                for (i, &ch) in pixel.0.iter().enumerate() {
                    sum_squared_diff[i] += (ch as f32 - mean[i]).powi(2);
                }
            }
            total_pixels += (image.width() * image.height()) as usize;
        }

        // Calculate variance and then take the square root for std
        sum_squared_diff.iter_mut().for_each(|s| {
            *s = (*s / total_pixels as f32).sqrt(); // Compute std deviation
        });
        sum_squared_diff
    }

    fn match_palette(&self, other: &impl ColorMatch) -> Vec<RgbImage> {
        let self_lab = self.to_lab();
        let other_lab = other.to_lab();

        let src_mean = self_lab.mean();
        let src_std = self_lab.std(&src_mean);
        let tgt_mean = other_lab.mean();
        let tgt_std = other_lab.std(&tgt_mean);

        // Apply palette matching to all images in the slice
        // by "merging" their color distributions
        self_lab
            .iter()
            .map(|image| match_palette_img(image, &src_mean, &tgt_mean, &src_std, &tgt_std))
            .collect::<Vec<_>>()
            .to_srgb()
    }

    fn equalize(&self) -> Vec<RgbImage> {
        // Create a combined histogram for all images
        // Three channels: R, G, B
        let mut histograms = [0; 256 * 3];
        for image in self.iter() {
            for pixel in image.pixels() {
                for (i, &ch) in pixel.0.iter().enumerate() {
                    histograms[i * 256 + ch as usize] += 1; // Increment the histogram
                }
            }
        }
        // Calculate cumulative distribution function
        let total_pixels = histograms.iter().sum::<usize>();
        // Three channels: R, G, B
        let cdfs = histograms_to_cdfs(&histograms);

        // Normalize CDF and equalize each image
        self.iter()
            .map(|img| equalize_img(img, &cdfs, &total_pixels))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::*;
    use image::open;

    fn test_dir() -> std::path::PathBuf {
        PathBuf::from_str("tests/data/").unwrap()
    }

    fn generate_test_image(width: u32, height: u32, color: [u8; 3]) -> RgbImage {
        ImageBuffer::from_fn(width, height, |_x, _y| Rgb(color))
    }

    #[test]
    fn test_mean_std() {
        // create 2x1 image with one white and one black pixel
        let img = ImageBuffer::from_fn(2, 1, |x, _| {
            if x == 0 {
                Rgb([0, 0, 0])
            } else {
                Rgb([255, 255, 255])
            }
        });

        assert_eq!(img.mean(), [127.5, 127.5, 127.5]);
        assert_eq!(img.std(&[127.5, 127.5, 127.5]), [127.5, 127.5, 127.5]);

        // now we test the Vec of img logic
        let imgs = vec![
            ImageBuffer::from_pixel(2, 2, Rgb([0, 0, 0])),
            ImageBuffer::from_pixel(2, 2, Rgb([255, 255, 255])),
        ];

        assert_eq!(imgs.mean(), [127.5, 127.5, 127.5]);
        assert_eq!(imgs.std(&[127.5, 127.5, 127.5]), [127.5, 127.5, 127.5]);
    }

    #[test]
    fn test_mean_uniform() {
        // Create a 2x2 image where all pixels are the same color
        let img = generate_test_image(2, 2, [100, 150, 200]);
        let mean = img.mean();
        assert_eq!(mean, [100.0, 150.0, 200.0], "Mean calculation failed");
    }

    #[test]
    fn test_std_uniform() {
        // Create a 2x2 image where all pixels are the same color, standard deviation should be 0
        let img = generate_test_image(2, 2, [100, 150, 200]);
        let mean = img.mean();
        let std = img.std(&mean);
        assert_eq!(
            std,
            [0.0, 0.0, 0.0],
            "Standard deviation calculation failed for uniform color"
        );
    }

    #[test]
    fn test_match_palette() {
        // Create two images with different colors
        let img1 = generate_test_image(2, 2, [50, 100, 150]);
        let img2 = generate_test_image(2, 2, [100, 150, 200]);
        // Match img1's palette to img2
        let matched_img = img1.match_palette(&img2);
        let matched_mean = matched_img.mean();
        let img2_mean = img2.mean();
        // Check that the color-matched image has a mean similar to the target (img2)
        assert!(
            matched_mean
                .iter()
                .zip(img2_mean.iter())
                .all(|(a, b)| (a - b).abs() < 5.0),
            "Color matching failed"
        );
    }

    #[test]
    fn test_equalize() {
        // Create an image with a gradient to test equalization
        // the image goes from black to gray
        let img = ImageBuffer::from_fn(256, 1, |x, _| Rgb([x as u8 / 2, x as u8 / 2, x as u8 / 2]));
        // Apply histogram equalization
        // the image should go from black to white
        let equalized_img = img.equalize();
        assert_eq!(
            *equalized_img.get_pixel(0, 0),
            Rgb([0, 0, 0]),
            "Equalization failed"
        );
        assert_eq!(
            *equalized_img.get_pixel(255, 0),
            Rgb([255, 255, 255]),
            "Equalization failed"
        );
    }

    #[test]
    fn test_equalize_with_real_image() {
        // Load the source and target images
        let test_dir = test_dir().join("match");
        let img = open(test_dir.join("target.png")).unwrap().to_rgb8();
        let img_equalized = img.equalize();

        if std::env::var("PHOMO_UPDATE_EXPECTED").is_ok() {
            img_equalized
                .save(test_dir.join("target_equalized.png"))
                .unwrap();
        }
        let expected_img = open(test_dir.join("target_equalized.png"))
            .unwrap()
            .to_rgb8();
        assert_eq!(img_equalized, expected_img, "Equalization failed");
    }

    #[test]
    fn test_match_palette_with_real_images() {
        // Load the source and target images
        let test_dir = test_dir().join("match");
        let source = open(test_dir.join("source.png")).unwrap().to_rgb8();
        let target = open(test_dir.join("target.png")).unwrap().to_rgb8();

        // Match sources's palette to target
        let matched_img = source.match_palette(&target);
        // needs to be lossless format to avoid artefacts
        let matched_file = test_dir.join("matched.png");

        // update expected images if PHOMO_UPDATE_EXPECTED is set
        if std::env::var("PHOMO_UPDATE_EXPECTED").is_ok() {
            matched_img.save(&matched_file).unwrap();
        }
        let expected_img = open(&matched_file).unwrap().to_rgb8();

        // Compare the matched image with the expected image
        let matched_pixels: Vec<u8> = matched_img.pixels().flat_map(|p| p.0).collect();
        let expected_pixels: Vec<u8> = expected_img.pixels().flat_map(|p| p.0).collect();
        assert_eq!(matched_pixels.len(), expected_pixels.len());
        assert_eq!(matched_pixels, expected_pixels);
    }

    // Color space tests
    const LAB_RED: [u8; 3] = [160, 185, 160];
    const LAB_GREEN: [u8; 3] = [221, 68, 173];
    const LAB_BLUE: [u8; 3] = [115, 119, 48];
    const LAB_WHITE: [u8; 3] = [255, 128, 128];

    #[test]
    fn test_rgb_to_lab_conversion() {
        // Create a small test image (2x2)
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(2, 2, |x, y| {
            if x == 0 && y == 0 {
                Rgb([255, 0, 0])
            } else if x == 1 && y == 0 {
                Rgb([0, 255, 0])
            } else if x == 0 && y == 1 {
                Rgb([0, 0, 255])
            } else {
                Rgb([255, 255, 255])
            }
        });

        // Convert the image to Lab space
        let lab_image = img.to_lab();

        // Check dimensions
        assert_eq!(lab_image.width(), 2);
        assert_eq!(lab_image.height(), 2);

        // Verify that the values were converted
        assert_eq!(lab_image.get_pixel(0, 0), &Rgb(LAB_RED)); // Should be close to Red
        assert_eq!(lab_image.get_pixel(1, 0), &Rgb(LAB_GREEN)); // Should be close to Green
        assert_eq!(lab_image.get_pixel(0, 1), &Rgb(LAB_BLUE)); // Should be close to Blue
        assert_eq!(lab_image.get_pixel(1, 1), &Rgb(LAB_WHITE)); // Should be close to White
    }

    #[test]
    fn test_lab_to_rgb_conversion() {
        // Create a small test image (2x2) in Lab space
        let lab_img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(2, 2, |x, y| {
            if x == 0 && y == 0 {
                Rgb(LAB_RED) // Approximate LAB for Red
            } else if x == 1 && y == 0 {
                Rgb(LAB_GREEN) // Approximate LAB for Green
            } else if x == 0 && y == 1 {
                Rgb(LAB_BLUE) // Approximate LAB for Blue
            } else {
                Rgb(LAB_WHITE) // Approximate LAB for White
            }
        });

        // Convert the Lab image back to RGB
        let rgb_image = lab_img.to_srgb();

        // Check dimensions
        assert_eq!(rgb_image.width(), 2);
        assert_eq!(rgb_image.height(), 2);

        // Verify pixel values, there are some errors
        assert_eq!(rgb_image.get_pixel(0, 0), &Rgb([255, 0, 0])); // Should be close to Red
        assert_eq!(rgb_image.get_pixel(1, 0), &Rgb([1, 255, 10])); // Should be close to Green
        assert_eq!(rgb_image.get_pixel(0, 1), &Rgb([0, 0, 255])); // Should be close to Blue
        assert_eq!(rgb_image.get_pixel(1, 1), &Rgb([255, 255, 254])); // Should be close to White
    }

    #[test]
    fn test_rgb_to_lab_and_back_conversion() {
        // Create a small test image (2x2) in RGB space
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(2, 2, |x, y| {
            if x == 0 && y == 0 {
                Rgb([255, 0, 0]) // Red
            } else if x == 1 && y == 0 {
                Rgb([0, 255, 0]) // Green
            } else if x == 0 && y == 1 {
                Rgb([0, 0, 255]) // Blue
            } else {
                Rgb([255, 255, 255]) // White
            }
        });

        // Convert to Lab space
        let lab_image = img.to_lab();
        // Convert back to RGB
        let rgb_image = lab_image.to_srgb();

        // Check dimensions
        assert_eq!(rgb_image.width(), 2);
        assert_eq!(rgb_image.height(), 2);

        let error: u8 = 20;
        // Verify that the conversion back and forth doesn't alter the colors too much
        assert!(rgb_image
            .pixels()
            .zip(img.pixels())
            .all(|(rgb, lab)| (rgb[0].abs_diff(lab[0]) < error
                && rgb[1].abs_diff(lab[1]) < error
                && rgb[2].abs_diff(lab[2]) < error)));
    }
}
