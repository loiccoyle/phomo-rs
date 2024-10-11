use std::path::Path;

extern crate image;
extern crate log;
use image::{GenericImageView, RgbImage};
use log::{debug, info};

use crate::color_space::{ToLab, ToRgb};
use crate::error::Error;
use crate::utils;

pub struct Master {
    pub img: RgbImage,
    pub cells: Vec<RgbImage>,
    pub cell_size: (u32, u32),
}

impl Master {
    pub fn from_image(img: RgbImage, grid_size: (u32, u32)) -> Result<Self, Error> {
        let (img_width, img_height) = img.dimensions();
        // the number of cells in each dimension of the grid
        let (grid_width, grid_height) = grid_size;
        // How much is left over around the grid
        let (width_extra, height_extra) = (img_width % grid_width, img_height % grid_height);

        let img = if width_extra > 0 || height_extra > 0 {
            info!("Cropping image to fit grid size");
            let img_cropped =
                utils::crop_imm_centered(&img, img_width - width_extra, img_height - height_extra);
            img_cropped.to_image()
        } else {
            img
        };
        debug!("Image dimensions: {}x{}", img.width(), img.height());

        let (cells, cell_size) = Self::construct_regions(&img, grid_size)?;
        Ok(Master {
            img,
            cells,
            cell_size,
        })
    }

    pub fn from_file<P: AsRef<Path>>(file: P, grid_size: (u32, u32)) -> Result<Self, Error> {
        let img = image::open(file)?.to_rgb8();
        Self::from_image(img, grid_size)
    }

    fn construct_regions(
        img: &RgbImage,
        grid_size: (u32, u32),
    ) -> Result<(Vec<RgbImage>, (u32, u32)), Error> {
        let (grid_width, grid_height) = grid_size;
        if img.width() % grid_width != 0 || img.height() % grid_height != 0 {
            return Err(
                "Invalid grid shape, the imge dimensions are not divisible by the grid shape"
                    .into(),
            );
        }

        let (cell_width, cell_height) = (img.width() / grid_width, img.height() / grid_height);
        info!("Grid Cell size: {}x{}", cell_width, cell_height);

        let cells = (0..grid_height)
            .flat_map(|y| {
                (0..grid_width).map(move |x| {
                    img.view(x * cell_width, y * cell_height, cell_width, cell_height)
                        .to_image()
                })
            })
            .collect();
        Ok((cells, (cell_width, cell_height)))
    }
}

impl ToLab for Master {
    type Output = Master;
    fn to_lab(&self) -> Master {
        Master {
            img: self.img.to_lab(),
            cells: self.cells.as_slice().to_lab(),
            cell_size: self.cell_size,
        }
    }
}

impl ToRgb for Master {
    type Output = Master;
    fn to_rgb(&self) -> Master {
        Master {
            img: self.img.to_rgb(),
            cells: self.cells.as_slice().to_rgb(),
            cell_size: self.cell_size,
        }
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
    fn test_master_img() -> PathBuf {
        // 256x256 image
        test_dir().join("master.png")
    }

    fn check_master(master: Master) {
        let test_dir = test_dir();

        // Assert that the image has been loaded
        assert_eq!(master.img.width(), 256);
        assert_eq!(master.img.height(), 256);

        // Assert that the cells have been correctly divided
        assert_eq!(master.cells.len(), 16);
        for region in &master.cells {
            assert_eq!(region.width(), 64);
            assert_eq!(region.height(), 64);
        }

        for (i, cell) in master.cells.iter().enumerate() {
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
        let image_path = test_master_img();
        let img = image::open(&image_path).unwrap().to_rgb8();
        let grid_size = (4, 4);
        let master = Master::from_image(img, grid_size).unwrap();

        check_master(master);
    }

    #[test]
    fn test_from_file_valid_image() {
        // 256x256 image
        let image_path = test_master_img();
        let grid_size = (4, 4);
        let master = Master::from_file(image_path, grid_size).unwrap();

        check_master(master);
    }

    #[test]
    fn test_from_image_non_divisible() {
        let image_path = test_master_img();
        let img = image::open(&image_path).unwrap().to_rgb8();
        let grid_size = (4, 4);

        let img_resized =
            image::imageops::resize(&img, 255, 255, image::imageops::FilterType::Lanczos3);

        let master = Master::from_image(img_resized, grid_size).unwrap();
        // Assert that the image has been loaded
        assert_eq!(master.img.width(), 252);
        assert_eq!(master.img.height(), 252);

        assert_eq!(master.cells.len(), 16);
        for region in &master.cells {
            assert_eq!(region.width(), 63);
            assert_eq!(region.height(), 63);
        }
    }

    #[test]
    fn test_construct_regions() {
        // Create a 256x256 test image
        let img = create_test_image(256, 256);
        let grid_size = (4, 4);

        let (cells, _) = Master::construct_regions(&img, grid_size).unwrap();
        // Assert that the cells have been divided correctly
        assert_eq!(cells.len(), 16); // 4x4 grid
        for region in &cells {
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
