use image::{ImageBuffer, Rgb};
use palette::{IntoColor, Oklab, Srgb};

// used to distinguish the color space in the signatures
type Lab = Rgb<u8>;

pub(crate) trait ToLab {
    type Output;
    fn to_lab(&self) -> Self::Output;
}

impl ToLab for ImageBuffer<Rgb<u8>, Vec<u8>> {
    type Output = ImageBuffer<Lab, Vec<u8>>;

    /// Convert to [`Oklab`] color space but mapped to [0, 255].
    fn to_lab(&self) -> Self::Output {
        // Create an iterator over the pixels and convert them to LAB
        let lab_pixels: Vec<Rgb<u8>> = self
            .pixels()
            .map(|pixel| {
                // Convert from RGB<u8> to linear RGB
                println!("{:?}", pixel);
                let lab: Oklab = Srgb::from([pixel[0], pixel[1], pixel[2]])
                    .into_linear()
                    .into_color();
                println!("{:?}", lab);
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

impl ToLab for &[ImageBuffer<Rgb<u8>, Vec<u8>>] {
    type Output = Vec<ImageBuffer<Lab, Vec<u8>>>;

    fn to_lab(&self) -> Self::Output {
        self.iter().map(|img| img.to_lab()).collect::<Vec<_>>()
    }
}

pub(crate) trait ToRgb {
    type Output;

    fn to_rgb(&self) -> Self::Output;
}

impl ToRgb for ImageBuffer<Lab, Vec<u8>> {
    type Output = ImageBuffer<Rgb<u8>, Vec<u8>>;

    fn to_rgb(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        // Create an iterator over the LAB pixels and convert them back to RGB
        let rgb_pixels: Vec<Rgb<u8>> = self
            .pixels()
            .map(|pixel| {
                // Convert LAB to linear RGB
                println!("{:?}", pixel);
                let lab = Oklab::new(
                    pixel[0] as f32 / 255.,
                    pixel[1] as f32 / 255. - 0.5,
                    pixel[2] as f32 / 255. - 0.5,
                );
                println!("{:?}", lab);
                let srgb: Srgb = Srgb::from_linear(lab.into_color());
                println!("{:?}", srgb);
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

impl ToRgb for &[ImageBuffer<Lab, Vec<u8>>] {
    type Output = Vec<ImageBuffer<Rgb<u8>, Vec<u8>>>;

    fn to_rgb(&self) -> Self::Output {
        self.iter().map(|img| img.to_rgb()).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
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
        let rgb_image = lab_img.to_rgb();

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
        let rgb_image = lab_image.to_rgb();

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
