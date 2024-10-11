extern crate image;
use image::RgbImage;

pub trait Metric {
    fn compute(&self, img1: &image::RgbImage, img2: &image::RgbImage) -> i64;
}

pub struct NormL1;
impl Metric for NormL1 {
    fn compute(&self, img1: &RgbImage, img2: &RgbImage) -> i64 {
        img1.pixels().zip(img2.pixels()).fold(0, |sum, (p1, p2)| {
            sum + (p1[0].abs_diff(p2[0]) as i64)
                + (p1[1].abs_diff(p2[1]) as i64)
                + (p1[2].abs_diff(p2[2]) as i64)
        })
    }
}

pub struct NormL2;
impl Metric for NormL2 {
    fn compute(&self, img1: &RgbImage, img2: &RgbImage) -> i64 {
        // l2 norm without the sqrt to stay an integer
        img1.pixels().zip(img2.pixels()).fold(0, |sum, (p1, p2)| {
            sum + (p1[0].abs_diff(p2[0]) as i64).pow(2)
                + (p1[1].abs_diff(p2[1]) as i64).pow(2)
                + (p1[2].abs_diff(p2[2]) as i64).pow(2)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_master_img() -> PathBuf {
        // image is 256x256
        PathBuf::from("tests/data/master/master.png")
    }

    #[test]
    fn test_norml1() {
        let img1 = image::open(test_master_img()).unwrap().to_rgb8();
        let img2 = img1.clone();
        assert_eq!(NormL1.compute(&img1, &img2), 0);

        let img1 = image::ImageBuffer::from_pixel(64, 64, image::Rgb([0, 0, 0]));
        let img2 = image::ImageBuffer::from_pixel(64, 64, image::Rgb([255, 255, 255]));
        assert_eq!(NormL1.compute(&img1, &img2), 64 * 64 * 255 * 3);
    }

    #[test]
    fn test_norml2() {
        let img1 = image::open(test_master_img()).unwrap().to_rgb8();
        let img2 = img1.clone();
        assert_eq!(NormL2.compute(&img1, &img2), 0);

        let img1 = image::ImageBuffer::from_pixel(64, 64, image::Rgb([0, 0, 0]));
        let img2 = image::ImageBuffer::from_pixel(64, 64, image::Rgb([255, 255, 255]));
        assert_eq!(NormL2.compute(&img1, &img2), 64 * 64 * 255_i64.pow(2) * 3);
    }
}
