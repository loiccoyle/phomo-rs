use crate::error::Error;
use std::path::Path;

extern crate image;
use image::{GenericImageView, RgbImage};

pub struct Master {
    img: RgbImage,
    regions: Vec<RgbImage>,
}

impl Master {
    fn from_image(img: RgbImage, grid_shape: (usize, usize)) -> Result<Self, Error> {
        let (img_width, img_height) = img.dimensions();
        let (grid_width, grid_height) = grid_shape;

        let (width_extra, height_extra) = (
            img_width % grid_width as u32,
            img_height % grid_height as u32,
        );
        let (top_left_x, top_left_y) = (width_extra / 2, height_extra / 2);

        let img_cropped = image::imageops::crop_imm(
            &img,
            top_left_x,
            top_left_y,
            img_width - width_extra,
            img_height - height_extra,
        );
        let img = img_cropped.to_image();
        let regions = Self::construct_regions(&img, grid_shape)?;

        Ok(Master { img, regions })
    }
    fn from_file<P: AsRef<Path>>(file: P, grid_shape: (usize, usize)) -> Result<Self, Error> {
        let img = image::open(file)?.to_rgb8();
        Self::from_image(img, grid_shape)
    }

    fn construct_regions(
        img: &RgbImage,
        grid_shape: (usize, usize),
    ) -> Result<Vec<RgbImage>, Error> {
        let (grid_width, grid_height) = grid_shape;
        if img.width() % grid_width as u32 != 0 || img.height() % grid_height as u32 != 0 {
            return Err("Invalid grid shape".into());
        }

        let (cell_width, cell_height) = (
            img.width() / grid_width as u32,
            img.height() / grid_height as u32,
        );

        let mut regions = Vec::with_capacity(grid_width * grid_height);

        for y in 0..grid_height {
            for x in 0..grid_width {
                let region = img.view(
                    x as u32 * cell_width,
                    y as u32 * cell_height,
                    cell_width,
                    cell_height,
                );
                regions.push(region.to_image());
            }
        }
        Ok(regions)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use image::RgbImage;

    // Helper function to create a simple test image
    fn create_test_image(width: u32, height: u32) -> RgbImage {
        let mut img = RgbImage::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = image::Rgb([(x % 256) as u8, (y % 256) as u8, 0]);
        }
        img
    }

    fn test_dir() -> PathBuf {
        PathBuf::from("tests/data/master")
    }

    fn check_master(master: Master) {
        let test_dir = test_dir();

        // Assert that the image has been loaded
        assert_eq!(master.img.width(), 256);
        assert_eq!(master.img.height(), 256);

        // Assert that the regions have been correctly divided
        assert_eq!(master.regions.len(), 16);
        for region in &master.regions {
            assert_eq!(region.width(), 64);
            assert_eq!(region.height(), 64);
        }

        for (i, cell) in master.regions.iter().enumerate() {
            let expected_file = test_dir.join(format!("cells/{:02}.png", i));
            if std::env::var("PHOMO_UPDATE_EXPECTED").is_ok() {
                cell.save(&expected_file).unwrap();
            }

            let expected_cell = image::open(&expected_file).unwrap().to_rgb8();
            assert_eq!(cell, &expected_cell);
        }
    }

    #[test]
    fn test_from_image_buffer() {
        let test_dir = test_dir();
        let image_path = test_dir.join("pfp.png");
        let img = image::open(&image_path).unwrap().to_rgb8();
        let grid_shape = (4, 4);
        let master = Master::from_image(img, grid_shape).unwrap();

        check_master(master);
    }

    #[test]
    fn test_from_file_valid_image() {
        // Path to a valid image file (replace with an actual image path for testing)
        // 256x256 image
        let test_dir = test_dir();
        let image_path = test_dir.join("pfp.png");
        let grid_shape = (4, 4);
        let master = Master::from_file(image_path, grid_shape).unwrap();

        check_master(master);
    }

    #[test]
    fn test_construct_regions() {
        // Create a 256x256 test image
        let img = create_test_image(256, 256);
        let grid_shape = (4, 4);

        let regions = Master::construct_regions(&img, grid_shape).unwrap();
        // Assert that the regions have been divided correctly
        assert_eq!(regions.len(), 16); // 4x4 grid
        for region in &regions {
            assert_eq!(region.width(), 64); // Each region should be 64x64
            assert_eq!(region.height(), 64);
        }
    }

    #[test]
    fn test_invalid_grid_shape() {
        let img = create_test_image(256, 256);
        // Test invalid grid shape (e.g., non-divisible dimensions)
        // 5 doesn't divide 256 evenly
        let result = Master::construct_regions(&img, (5, 4));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_file_path() {
        // Try loading from a nonexistent file
        let result = Master::from_file("invalid/path/to/file.png", (4, 4));
        // Assert that an error is returned
        assert!(result.is_err());
    }
}
