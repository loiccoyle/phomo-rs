use image::RgbImage;

pub(crate) fn norm_l1(img1: &RgbImage, img2: &RgbImage) -> i64 {
    let data1 = img1.as_raw();
    let data2 = img2.as_raw();

    data1
        .iter()
        .zip(data2.iter())
        .map(|(byte1, byte2)| i64::from(byte1.abs_diff(*byte2)))
        .sum()
}

pub(crate) fn norm_l2(img1: &RgbImage, img2: &RgbImage) -> i64 {
    let data1 = img1.as_raw();
    let data2 = img2.as_raw();

    data1
        .iter()
        .zip(data2.iter())
        .map(|(byte1, byte2)| {
            let diff = i64::from(byte1.abs_diff(*byte2));
            diff * diff
        })
        .sum::<i64>()
        .isqrt()
}
