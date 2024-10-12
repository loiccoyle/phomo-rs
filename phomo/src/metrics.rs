extern crate image;
use image::RgbImage;

pub(crate) type MetricFn = fn(&RgbImage, &RgbImage) -> i64;

/// L1 norm, the sum of the absolute differences.
///
/// # Examples
///
/// ```rust
/// use phomo::metrics::norm_l1;
/// use image;
///
/// let img1 = image::ImageBuffer::from_pixel(2, 2, image::Rgb([0, 0, 0]));
/// let img2 = image::ImageBuffer::from_pixel(2, 2, image::Rgb([255, 255, 255]));
/// let norm = norm_l1(&img1, &img2);
/// assert_eq!(norm, 255 * 3 * 2 * 2);
/// ```
pub fn norm_l1(img1: &RgbImage, img2: &RgbImage) -> i64 {
    img1.pixels().zip(img2.pixels()).fold(0, |sum, (p1, p2)| {
        sum + (p1[0].abs_diff(p2[0]) as i64)
            + (p1[1].abs_diff(p2[1]) as i64)
            + (p1[2].abs_diff(p2[2]) as i64)
    })
}

/// L2 norm, but without the square root to stay an integer.
///
/// # Examples
///
/// ```rust
/// use phomo::metrics::norm_l2;
/// use image;
///
/// let img1 = image::ImageBuffer::from_pixel(2, 2, image::Rgb([0, 0, 0]));
/// let img2 = image::ImageBuffer::from_pixel(2, 2, image::Rgb([255, 255, 255]));
/// let norm = norm_l2(&img1, &img2);
/// assert_eq!(norm, 255_i64.pow(2) * 3 * 2 * 2);
/// ```
pub fn norm_l2(img1: &RgbImage, img2: &RgbImage) -> i64 {
    // l2 norm without the sqrt to stay an integer
    img1.pixels().zip(img2.pixels()).fold(0, |sum, (p1, p2)| {
        sum + (p1[0].abs_diff(p2[0]) as i64).pow(2)
            + (p1[1].abs_diff(p2[1]) as i64).pow(2)
            + (p1[2].abs_diff(p2[2]) as i64).pow(2)
    })
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
        assert_eq!(norm_l1(&img1, &img2), 0);

        let img1 = image::ImageBuffer::from_pixel(64, 64, image::Rgb([0, 0, 0]));
        let img2 = image::ImageBuffer::from_pixel(64, 64, image::Rgb([255, 255, 255]));
        assert_eq!(norm_l1(&img1, &img2), 64 * 64 * 255 * 3);
    }

    #[test]
    fn test_norml2() {
        let img1 = image::open(test_master_img()).unwrap().to_rgb8();
        let img2 = img1.clone();
        assert_eq!(norm_l2(&img1, &img2), 0);

        let img1 = image::ImageBuffer::from_pixel(64, 64, image::Rgb([0, 0, 0]));
        let img2 = image::ImageBuffer::from_pixel(64, 64, image::Rgb([255, 255, 255]));
        assert_eq!(norm_l2(&img1, &img2), 64 * 64 * 255_i64.pow(2) * 3);
    }
}
