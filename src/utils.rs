extern crate image;
use image::{GenericImageView, SubImage};

/// Helper function to crop am image to a width and height centered on the image
pub(crate) fn crop_imm_centered<I>(img: &I, width: u32, height: u32) -> SubImage<&I>
where
    I: GenericImageView,
{
    image::imageops::crop_imm(
        img,
        (img.width() - width) / 2,
        (img.height() - height) / 2,
        width,
        height,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crop_imm_centered() {
        // create white image with a black pixel centered on the image
        let mut img = image::RgbImage::from_pixel(11, 11, image::Rgb([255, 255, 255]));
        img.put_pixel(5, 5, image::Rgb([0, 0, 0]));

        let cropped = crop_imm_centered(&img, 5, 5);
        assert_eq!(cropped.dimensions(), (5, 5));
        assert_eq!(cropped.get_pixel(2, 2), image::Rgb([0, 0, 0]));

        let cropped = crop_imm_centered(&img, 1, 1);
        assert_eq!(cropped.dimensions(), (1, 1));
        assert_eq!(cropped.get_pixel(0, 0), image::Rgb([0, 0, 0]));
    }
}
