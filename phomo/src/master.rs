use std::path::Path;

extern crate image;
use image::{GenericImage, GenericImageView, Rgb, RgbImage};
use log::{debug, info};

use crate::error::Error;
use crate::utils;

#[derive(Debug, Clone)]
pub struct Master {
    /// The master image buffer.
    pub img: RgbImage,
    /// The grid cells of the master image, where the tiles will be placed.
    pub cells: Vec<RgbImage>,
    /// The size of each grid cell, in pixels.
    pub cell_size: (u32, u32),
    grid_size: (u32, u32),
}

/// Represents the master image.
///
/// The image will be split in to [`cells`](Master::cells), wihch contain the smaller regions of
/// the master image, where the tiles will be placed.
impl Master {
    /// Construct a [`Master`] from a [`RgbImage`] buffer, and the grid size.
    ///
    /// # Arguments
    /// - `img`: The [`RgbImage`] buffer to construct the [`Master`] from.
    /// - `grid_size`: The grid size of the [`Master`], the number of cells horizontally and vertically.
    pub fn from_image(img: RgbImage, grid_size: (u32, u32)) -> Result<Self, Error> {
        let (img_width, img_height) = img.dimensions();
        // the number of cells in each dimension of the grid
        let (grid_width, grid_height) = grid_size;
        let (cell_width, cell_height) = (img_width / grid_width, img_height / grid_height);
        info!("Grid Cell size: {}x{}", cell_width, cell_height);
        // How much is left over around the grid
        let (width_extra, height_extra) = (img_width % grid_width, img_height % grid_height);

        let img = if width_extra > 0 || height_extra > 0 {
            info!("Cropping image to fit grid");
            let img_cropped =
                utils::crop_imm_centered(&img, img_width - width_extra, img_height - height_extra);
            img_cropped.to_image()
        } else {
            img
        };
        debug!("Image dimensions: {}x{}", img.width(), img.height());

        let cells = Self::construct_regions(&img, grid_size)?;
        Ok(Master {
            img,
            cells,
            cell_size: (cell_width, cell_height),
            grid_size,
        })
    }

    /// Construct a [`Master`] from an image file, and the grid size.
    ///
    /// # Arguments
    /// - `file`: The path to the image file to construct the [`Master`] from.
    /// - `grid_size`: The grid size of the [`Master`], the number of cells horizontally and vertically.
    pub fn from_file<P: AsRef<Path>>(file: P, grid_size: (u32, u32)) -> Result<Self, Error> {
        let img = image::open(file)?.to_rgb8();
        Self::from_image(img, grid_size)
    }

    fn construct_regions(img: &RgbImage, grid_size: (u32, u32)) -> Result<Vec<RgbImage>, Error> {
        let (grid_width, grid_height) = grid_size;
        let (cell_width, cell_height) = (img.width() / grid_width, img.height() / grid_height);
        if img.width() % grid_width != 0 || img.height() % grid_height != 0 {
            return Err(
                format!("Invalid grid shape, the imge dimensions: {}x{} are not divisible by the cell size: {}x{}", 
                    img.width(), img.height(), cell_width, cell_height)
                    .into(),
            );
        }

        let cells = (0..grid_height)
            .flat_map(|y| {
                (0..grid_width).map(move |x| {
                    img.view(x * cell_width, y * cell_height, cell_width, cell_height)
                        .to_image()
                })
            })
            .collect();
        Ok(cells)
    }

    /// Overlay the grid onto the master image.
    pub fn overlay_grid(&self) -> Result<RgbImage, Error> {
        let (grid_width, grid_height) = self.grid_size;
        let mut grid_img = RgbImage::from_pixel(
            self.img.width() + grid_width - 1,
            self.img.height() + grid_height - 1,
            Rgb([255, 255, 255]),
        );

        for (i, region) in self.cells.iter().enumerate() {
            let col_i = i as u32 % grid_width;
            let row_i = i as u32 / grid_width;
            let x = col_i * self.cell_size.0 + col_i;
            let y = row_i * self.cell_size.1 + row_i;
            grid_img.copy_from(region, x, y)?;
        }

        Ok(grid_img)
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

        let cells = Master::construct_regions(&img, grid_size).unwrap();
        // Assert that the cells have been divided correctly
        assert_eq!(cells.len(), 16); // 4x4 grid
        for region in &cells {
            assert_eq!(region.width(), 64); // Each region should be 64x64
            assert_eq!(region.height(), 64);
        }
    }

    #[test]
    fn test_overlay_grid() {
        let img = create_test_image(128, 256);
        let grid_size = (4, 4);
        let master = Master::from_image(img, grid_size).unwrap();
        let grid_overlay = master.overlay_grid();
        assert!(grid_overlay.is_ok());
        let grid_overlay = grid_overlay.unwrap();

        let expected_path = test_dir().join("grid_overlay.png");
        if std::env::var("PHOMO_UPDATE_EXPECTED").is_ok() {
            grid_overlay.save(&expected_path).unwrap();
        }
        let expected_img = image::open(expected_path).unwrap().to_rgb8();
        assert_eq!(expected_img, grid_overlay);
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
