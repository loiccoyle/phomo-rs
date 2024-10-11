use std::path::Path;

extern crate image;
use image::{GenericImageView, RgbImage, SubImage};

use crate::error::Error;

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

/// Read all images in a directory and returns them as a vector
pub fn read_images_from_dir<P: AsRef<Path>>(tile_dir: P) -> Result<Vec<RgbImage>, Error> {
    Ok(tile_dir
        .as_ref()
        .read_dir()?
        .filter_map(|path| match path {
            Ok(p) => image::open(p.path()).map(|img| img.to_rgb8()).ok(),
            Err(_) => None,
        })
        .collect::<Vec<_>>())
}

/// Read all images in a directory, cropped to the `width` and `height` and returns them as a vector
pub fn read_images_from_dir_cropped<P: AsRef<Path>>(
    tile_dir: P,
    width: u32,
    height: u32,
) -> Result<Vec<RgbImage>, Error> {
    Ok(read_images_from_dir(tile_dir)?
        .iter()
        .map(|img| crop_imm_centered(img, width, height).to_image())
        .collect::<Vec<_>>())
}

/// Read all images in a directory, resized to the `width` and `height` and returns them as a vector
pub fn read_images_from_dir_resized<P: AsRef<Path>>(
    tile_dir: P,
    width: u32,
    height: u32,
    filter: image::imageops::FilterType,
) -> Result<Vec<RgbImage>, Error> {
    Ok(read_images_from_dir(tile_dir)?
        .iter()
        .map(|img| image::imageops::resize(img, width, height, filter))
        .collect::<Vec<_>>())
}

macro_rules! iter_or_par_iter {
    ($iter:expr) => {{
        #[cfg(feature = "parallel")]
        {
            $iter.par_iter()
        }
        #[cfg(not(feature = "parallel"))]
        {
            $iter.iter()
        }
    }};
}

pub(crate) use iter_or_par_iter;

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
