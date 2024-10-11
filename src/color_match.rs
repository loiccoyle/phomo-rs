extern crate image;
use image::{ImageBuffer, Rgb, RgbImage};

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

pub trait ColorMatch {
    type Output;

    fn mean(&self) -> [f32; 3];
    fn std(&self, mean: &[f32; 3]) -> [f32; 3];
    fn match_palette(&self, other: &impl ColorMatch) -> Self::Output;
    fn equalize(&self) -> Self::Output;
}

impl ColorMatch for RgbImage {
    type Output = RgbImage;

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
        let src_mean = self.mean();
        let src_std = self.std(&src_mean);
        let tgt_mean = other.mean();
        let tgt_std = other.std(&tgt_mean);
        match_palette_img(self, &src_mean, &tgt_mean, &src_std, &tgt_std)
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

// Implement ColorMatch for a slice of RgbImage
impl<'a> ColorMatch for &'a [RgbImage] {
    type Output = Vec<RgbImage>;

    fn mean(&self) -> [f32; 3] {
        let mut sum = [0.0; 3];
        let mut total_pixels = 0;

        for image in *self {
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

        for image in *self {
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
        let src_mean = self.mean();
        let src_std = self.std(&src_mean);
        let tgt_mean = other.mean();
        let tgt_std = other.std(&tgt_mean);

        // Apply palette matching to all images in the slice
        // by "merging" their color distributions
        self.iter()
            .map(|image| match_palette_img(image, &src_mean, &tgt_mean, &src_std, &tgt_std))
            .collect()
    }

    fn equalize(&self) -> Vec<RgbImage> {
        // Create a combined histogram for all images
        // Three channels: R, G, B
        let mut histograms = [0; 256 * 3];
        for image in *self {
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
                .all(|(a, b)| (a - b).abs() < 1.0),
            "Color matching failed"
        );
    }

    #[test]
    fn test_equalize() {
        // Create an image with a gradient to test equalization
        // the image goes from balck to gray
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
    fn test_match_palette_with_real_images() {
        // Load the source and target images
        let test_dir = test_dir().join("match");
        let source = open(test_dir.join("source.jpg")).unwrap().to_rgb8();
        let target = open(test_dir.join("target.jpg")).unwrap().to_rgb8();

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
        // // Assert that the matched image pixels are close to the expected image pixels
        // let tolerance = 5; // Allow some tolerance for color differences
        // for (matched, expected) in matched_pixels.iter().zip(expected_pixels.iter()) {
        //     assert!(
        //         (matched.abs_diff(*expected)) <= tolerance,
        //         "Color mismatch: matched: {}, expected: {}",
        //         matched,
        //         expected
        //     );
        // }
    }
}
