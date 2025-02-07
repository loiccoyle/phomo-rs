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
    img1.pixels()
        .zip(img2.pixels())
        .fold(0, |sum, (p1, p2)| {
            sum + (p1[0].abs_diff(p2[0]) as i64).pow(2)
                + (p1[1].abs_diff(p2[1]) as i64).pow(2)
                + (p1[2].abs_diff(p2[2]) as i64).pow(2)
        })
        .isqrt()
}
