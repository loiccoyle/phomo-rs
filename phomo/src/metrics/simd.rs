#[cfg(feature = "simd")]
use std::simd::{
    num::{SimdInt, SimdUint},
    Simd,
};

extern crate image;
use image::RgbImage;

const SIMD_LANES: usize = 64;

pub(crate) fn norm_l1(img1: &RgbImage, img2: &RgbImage) -> i64 {
    let data1 = img1.as_raw();
    let data2 = img2.as_raw();

    let len = data1.len();
    let mut sum: i64 = 0;
    let mut i = 0;

    // compute in simd buffers
    while i + SIMD_LANES <= len {
        let a: Simd<i16, SIMD_LANES> = Simd::from_slice(&data1[i..i + SIMD_LANES]).cast();
        let b: Simd<i16, SIMD_LANES> = Simd::from_slice(&data2[i..i + SIMD_LANES]).cast();
        sum += (a - b).abs().reduce_sum() as i64;
        i += SIMD_LANES;
    }

    // handle the remaining left over data which doesn't fit in a SIMD_LANES sized buffer
    while i < len {
        sum += data1[i].abs_diff(data2[i]) as i64;
        i += 1;
    }

    sum
}

pub(crate) fn norm_l2(img1: &RgbImage, img2: &RgbImage) -> i64 {
    let data1 = img1.as_raw();
    let data2 = img2.as_raw();

    let len = data1.len();
    let mut sum: i64 = 0;
    let mut i = 0;

    while i + SIMD_LANES <= len {
        let a: Simd<i16, SIMD_LANES> = Simd::from_slice(&data1[i..i + SIMD_LANES]).cast();
        let b: Simd<i16, SIMD_LANES> = Simd::from_slice(&data2[i..i + SIMD_LANES]).cast();
        let diff = a - b;
        sum += (diff * diff).reduce_sum() as i64;
        i += SIMD_LANES;
    }

    while i < len {
        let diff = data1[i].abs_diff(data2[i]) as i64;
        sum += diff * diff;
        i += 1;
    }

    sum.isqrt()
}
