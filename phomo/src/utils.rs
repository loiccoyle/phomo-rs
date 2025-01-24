use std::path::Path;

extern crate image;
use image::{GenericImageView, Pixel, RgbImage, SubImage};
use log::warn;

use crate::error::PhomoError;

/// Helper function to crop am image to a width and height centered on the image.
///
/// # Arguments
/// - `img`: The image to crop.
/// - `width`: The width to crop to.
/// - `height`: The height to crop to.
pub fn crop_imm_centered<I>(img: &I, width: u32, height: u32) -> SubImage<&I>
where
    I: GenericImageView,
{
    if width > img.width() || height > img.height() {
        warn!("Attempted to crop image to a larger size than the source image");
    }
    image::imageops::crop_imm(
        img,
        (img.width() - width) / 2,
        (img.height() - height) / 2,
        width,
        height,
    )
}

/// Helper function to crop an image, preserving aspect ratio, to a width and height.
///
/// # Arguments
/// - `img`: The image to crop.
/// - `width`: The width to crop to.
/// - `height`: The height to crop to.
/// - `filter`: The filter to use when resizing the image.
pub fn crop_cover<I>(
    img: &I,
    width: u32,
    height: u32,
    filter: image::imageops::FilterType,
) -> image::ImageBuffer<
    <I as GenericImageView>::Pixel,
    Vec<<<I as GenericImageView>::Pixel as Pixel>::Subpixel>,
>
where
    I: GenericImageView,
    <I as GenericImageView>::Pixel: 'static,
{
    let img_aspect_ratio = img.width() as f32 / img.height() as f32;
    let aspect_ratio = width as f32 / height as f32;

    let (new_width, new_height) = if img_aspect_ratio > aspect_ratio {
        // Tile is wider than the cell, so resize based on height
        let scale_factor = height as f32 / img.height() as f32;
        (img.width() as f32 * scale_factor, height as f32)
    } else {
        // Tile is taller than the cell, so resize based on width
        let scale_factor = width as f32 / img.width() as f32;
        (width as f32, img.height() as f32 * scale_factor)
    };

    let resized_img = image::imageops::resize(
        img,
        new_width.round() as u32,
        new_height.round() as u32,
        filter,
    );

    // Crop the resized image to the exact cell size, centered
    crop_imm_centered(&resized_img, width, height).to_image()
}

/// Read all images in a directory and returns them in a vector.
///
/// # Arguments
/// - `tile_dir`: The path to the directory containing the tile images.
///
/// # Errors
/// - An error occurred while reading the directory.
/// - Failed to open the image.
pub fn read_images_from_dir<P: AsRef<Path>>(tile_dir: P) -> Result<Vec<RgbImage>, PhomoError> {
    Ok(tile_dir
        .as_ref()
        .read_dir()?
        .filter_map(|entry| match entry {
            Ok(p) => match image::open(p.path()) {
                Ok(img) => Some(img.to_rgb8()),
                Err(e) => {
                    warn!("Failed to open image at path {:?}: {:?}", p.path(), e);
                    None
                }
            },
            Err(e) => {
                warn!("Failed to read directory entry: {:?}", e);
                None
            }
        })
        .collect::<Vec<_>>())
}

/// Read all images in a directory, cropped to the `width` and `height` and return them in a
/// vector.
///
/// # Arguments
/// - `tile_dir`: The path to the directory containing the tile images.
/// - `width`: The width to crop to.
/// - `height`: The height to crop to.
pub fn read_images_from_dir_cropped<P: AsRef<Path>>(
    tile_dir: P,
    width: u32,
    height: u32,
) -> Result<Vec<RgbImage>, PhomoError> {
    Ok(read_images_from_dir(tile_dir)?
        .iter()
        .map(|img| crop_cover(img, width, height, image::imageops::FilterType::Nearest))
        .collect::<Vec<_>>())
}

/// Read all images in a directory, resized to the `width` and `height` and returns them in a vector.
///
/// # Arguments
/// - `tile_dir`: The path to the directory containing the tile images.
/// - `width`: The width to resize to.
/// - `height`: The height to resize to.
/// - `filter`: The [`image::imageops::FilterType`] to use for resizing.
pub fn read_images_from_dir_resized<P: AsRef<Path>>(
    tile_dir: P,
    width: u32,
    height: u32,
    filter: image::imageops::FilterType,
) -> Result<Vec<RgbImage>, PhomoError> {
    Ok(read_images_from_dir(tile_dir)?
        .iter()
        .map(|img| image::imageops::resize(img, width, height, filter))
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_dir() -> PathBuf {
        PathBuf::from("tests/data/mosaic")
    }

    fn test_tile_dir() -> PathBuf {
        // tiles are 64x64
        test_dir().join("tiles/")
    }

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

    #[test]
    fn test_read_images_from_dir() {
        let images = read_images_from_dir(test_tile_dir()).unwrap();
        assert_eq!(images.len(), 16);
        assert!(images.iter().all(|img| img.dimensions() == (64, 64)));
    }

    #[test]
    fn test_read_images_from_dir_cropped() {
        let (width, height) = (32, 32);
        let images = read_images_from_dir_cropped(test_tile_dir(), width, height).unwrap();
        assert_eq!(images.len(), 16);
        assert!(images.iter().all(|img| img.dimensions() == (width, height)));
    }

    #[test]
    fn test_read_images_from_dir_resized() {
        let (width, height) = (32, 32);
        let images = read_images_from_dir_resized(
            test_tile_dir(),
            width,
            height,
            image::imageops::FilterType::Nearest,
        )
        .unwrap();
        assert_eq!(images.len(), 16);
        assert!(images.iter().all(|img| img.dimensions() == (width, height)));
    }
}
