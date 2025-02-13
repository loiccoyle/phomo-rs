extern crate image;
use image::RgbImage;

#[cfg(feature = "simd")]
#[path = "simd.rs"]
mod maybe_simd;
#[cfg(not(feature = "simd"))]
#[path = "default.rs"]
mod maybe_simd;

pub(crate) type MetricFn = fn(&RgbImage, &RgbImage) -> i64;

/// L1 norm, the sum of the absolute differences of the pixels.
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
    maybe_simd::norm_l1(img1, img2)
}

/// L2 norm, the euclidean distance of the pixels.
///
/// Punishes mismatched image regions more than the L1 norm.
/// Leading to less contrasty mosaics.
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
/// assert_eq!(norm, (255_i64.pow(2) * 3 * 2 * 2).isqrt());
/// ```
pub fn norm_l2(img1: &RgbImage, img2: &RgbImage) -> i64 {
    maybe_simd::norm_l2(img1, img2)
}

#[inline]
fn luminance(pixel: &[u8; 3]) -> i64 {
    (0.299 * pixel[0] as f64 + 0.587 * pixel[1] as f64 + 0.114 * pixel[2] as f64) as i64
}

/// L1 norm of the luminance of the pixels.
///
/// # Examples
///
/// ```rust
/// use phomo::metrics::luminance_l1;
/// use image;
///
/// let img1 = image::ImageBuffer::from_pixel(2, 2, image::Rgb([0, 0, 0]));
/// let img2 = image::ImageBuffer::from_pixel(2, 2, image::Rgb([255, 255, 255]));
/// let norm = luminance_l1(&img1, &img2);
/// assert_eq!(norm, 255 * 2 * 2);
/// ```
pub fn luminance_l1(img1: &RgbImage, img2: &RgbImage) -> i64 {
    img1.pixels().zip(img2.pixels()).fold(0, |sum, (p1, p2)| {
        let lum1 = luminance(&p1.0);
        let lum2 = luminance(&p2.0);
        sum + (lum1 - lum2).abs()
    })
}

/// L2 norm of the luminance of the pixels.
///
/// # Examples
///
/// ```rust
/// use phomo::metrics::luminance_l2;
/// use image;
///
/// let img1 = image::ImageBuffer::from_pixel(2, 2, image::Rgb([0, 0, 0]));
/// let img2 = image::ImageBuffer::from_pixel(2, 2, image::Rgb([255, 255, 255]));
/// let norm = luminance_l2(&img1, &img2);
/// assert_eq!(norm, (255_i64.pow(2) * 2 * 2).isqrt());
/// ```
pub fn luminance_l2(img1: &RgbImage, img2: &RgbImage) -> i64 {
    img1.pixels()
        .zip(img2.pixels())
        .fold(0, |sum, (p1, p2)| {
            let lum1 = luminance(&p1.0);
            let lum2 = luminance(&p2.0);
            sum + (lum1 - lum2).abs().pow(2)
        })
        .isqrt()
}

/// Difference of the average color of the images.
///
/// The distribution of the color is ignored, only the average color is used.
/// Preserves colors better but loses a lot of the details.
///
/// # Examples
///
/// ```rust
/// use phomo::metrics::avg_color;
/// use image;
///
/// let img1 = image::ImageBuffer::from_pixel(2, 2, image::Rgb([0, 0, 0]));
/// let img2 = image::ImageBuffer::from_pixel(2, 2, image::Rgb([255, 255, 255]));
/// let norm = avg_color(&img1, &img2);
/// assert_eq!(norm, 255);
/// ```
pub fn avg_color(img1: &RgbImage, img2: &RgbImage) -> i64 {
    let avg1 = img1.pixels().fold((0, 0, 0), |(r1, g1, b1), p1| {
        (r1 + p1[0] as i64, g1 + p1[1] as i64, b1 + p1[2] as i64)
    });
    let avg2 = img2.pixels().fold((0, 0, 0), |(r2, g2, b2), p2| {
        (r2 + p2[0] as i64, g2 + p2[1] as i64, b2 + p2[2] as i64)
    });

    (avg1.0.abs_diff(avg2.0) + avg1.1.abs_diff(avg2.1) + avg1.2.abs_diff(avg2.2)) as i64
        / (3 * img1.width() as i64 * img1.height() as i64)
}
