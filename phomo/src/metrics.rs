extern crate image;
use image::RgbImage;

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
    // |a - b| on widened bytes compiles to SAD instructions; u32 chunk sums can't overflow.
    raw_chunks(img1, img2, 1 << 16)
        .map(|(a, b)| {
            a.iter()
                .zip(b)
                .map(|(&x, &y)| (x as i32 - y as i32).unsigned_abs())
                .sum::<u32>() as i64
        })
        .sum()
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
    // Squares are at most 255^2, so 2^15 of them fit in a u32.
    raw_chunks(img1, img2, 1 << 15)
        .map(|(a, b)| {
            a.iter()
                .zip(b)
                .map(|(&x, &y)| (x as i32 - y as i32).pow(2) as u32)
                .sum::<u32>() as i64
        })
        .sum::<i64>()
        .isqrt()
}

/// The two images' pixel bytes, in matching chunks of `size`.
fn raw_chunks<'a>(
    img1: &'a RgbImage,
    img2: &'a RgbImage,
    size: usize,
) -> impl Iterator<Item = (&'a [u8], &'a [u8])> {
    let len = |img: &RgbImage| img.width() as usize * img.height() as usize * 3;
    let (a, b) = (&img1.as_raw()[..len(img1)], &img2.as_raw()[..len(img2)]);
    a.chunks(size).zip(b.chunks(size))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norms_match_per_pixel_sums() {
        let noise = |w, h, seed: u32| {
            RgbImage::from_fn(w, h, |x, y| {
                let n = (x.wrapping_mul(1664525) ^ y.wrapping_mul(1013904223) ^ seed)
                    .wrapping_mul(2891336453);
                image::Rgb([(n >> 24) as u8, (n >> 16) as u8, (n >> 8) as u8])
            })
        };
        for (w, h) in [(1, 1), (7, 3), (160, 120), (300, 200)] {
            let (a, b) = (noise(w, h, 1), noise(w, h, 2));
            let l1 = a.pixels().zip(b.pixels()).fold(0i64, |s, (p, q)| {
                s + (0..3).map(|c| p[c].abs_diff(q[c]) as i64).sum::<i64>()
            });
            let l2 = a.pixels().zip(b.pixels()).fold(0i64, |s, (p, q)| {
                s + (0..3).map(|c| (p[c].abs_diff(q[c]) as i64).pow(2)).sum::<i64>()
            });
            assert_eq!(norm_l1(&a, &b), l1);
            assert_eq!(norm_l2(&a, &b), l2.isqrt());
        }
    }
}
