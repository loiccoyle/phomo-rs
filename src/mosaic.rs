use std::path::Path;

use image::RgbImage;

use crate::error::Error;
use crate::master::Master;

fn read_tiles_from_dir<P: AsRef<Path>>(tile_dir: P) -> Result<Vec<RgbImage>, Error> {
    Ok(tile_dir
        .as_ref()
        .read_dir()?
        .filter_map(|path| match path {
            Ok(p) => image::open(p.path()).map(|img| img.to_rgb8()).ok(),
            Err(_) => None,
        })
        .collect::<Vec<_>>())
}

pub struct Mosaic {
    master: Master,
    tiles: Vec<RgbImage>,
    grid_size: (u32, u32),
}

impl Mosaic {
    pub fn from_file_and_dir<P: AsRef<Path>, Q: AsRef<Path>>(
        master_file: P,
        tile_dir: Q,
        grid_size: (u32, u32),
    ) -> Result<Self, Error> {
        let master_img = image::open(master_file)?.to_rgb8();
        let tiles = read_tiles_from_dir(tile_dir)?;

        Self::from_images(master_img, tiles, grid_size)
    }

    pub fn from_images(
        master_img: RgbImage,
        tiles: Vec<RgbImage>,
        grid_size: (u32, u32),
    ) -> Result<Self, Error> {
        let master = Master::from_image(master_img, grid_size)?;

        if tiles.iter().any(|img| img.dimensions() != master.cell_size) {
            return Err(format!(
                "All tiles must be the same size as the grid cells: {}x{}",
                master.cell_size.0, master.cell_size.1
            )
            .into());
        }

        Ok(Self {
            master,
            tiles,
            grid_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_dir() -> PathBuf {
        PathBuf::from("tests/data/mosaic")
    }
    fn test_master_img() -> PathBuf {
        // image is 256x256
        PathBuf::from("tests/data/master/master.png")
    }

    fn test_tile_dir() -> PathBuf {
        test_dir().join("tiles/")
    }

    #[test]
    fn test_mosaic_creation_from_valid_data() {
        let grid_size = (4, 4); // 4x4 grid
                                // Create a mosaic from the master image and the tile directory
        let mosaic = Mosaic::from_file_and_dir(test_master_img(), test_tile_dir(), grid_size);
        // Check if the mosaic was created successfully
        assert!(mosaic.is_ok());
        let mosaic = mosaic.unwrap();
        // Check that the master image has the expected dimensions
        assert_eq!(mosaic.master.img.width(), 256);
        assert_eq!(mosaic.master.img.height(), 256);
        // Check that the number of tiles matches the number of grid cells
        assert_eq!(mosaic.tiles.len(), 16);
    }

    #[test]
    fn test_mosaic_creation_with_mismatched_tile_sizes() {
        // 5x5 grid which will not work with a 256x256 master image and
        // 64x64 tiles
        let grid_size = (5, 5);
        // Attempt to create a mosaic and expect an error due to tile size mismatch
        let mosaic = Mosaic::from_file_and_dir(test_master_img(), test_tile_dir(), grid_size);
        assert!(mosaic.is_err());
    }

    #[test]
    fn test_invalid_master_file_path() {
        let grid_size = (4, 4); // 4x4 grid
                                // Attempt to create a mosaic with an invalid master file path
        let mosaic = Mosaic::from_file_and_dir("invalid/master.png", test_tile_dir(), grid_size);
        assert!(mosaic.is_err());
    }

    #[test]
    fn test_invalid_tile_directory() {
        // Create a valid master image
        let grid_size = (4, 4); // 4x4 grid (each cell should be 64x64)
                                // Attempt to create a mosaic with an invalid tile directory
        let mosaic = Mosaic::from_file_and_dir(test_master_img(), "invalid/tile_dir", grid_size);
        assert!(mosaic.is_err());
    }
}
