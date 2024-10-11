use std::path::Path;
use std::time;

extern crate image;
extern crate pathfinding;
use image::{GenericImage, RgbImage};
use log::info;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::error::Error;
use crate::master::Master;
use crate::metrics::{Metric, NormL1};
use crate::utils;

#[derive(Debug, Clone)]
pub struct Mosaic {
    pub master: Master,
    pub tiles: Vec<RgbImage>,
    grid_size: (u32, u32),
}

impl Mosaic {
    pub fn from_file_and_dir<P: AsRef<Path>, Q: AsRef<Path>>(
        master_file: P,
        tile_dir: Q,
        grid_size: (u32, u32),
    ) -> Result<Self, Error> {
        let master_img = image::open(master_file)?.to_rgb8();
        info!("Loading tiles");
        let tiles = utils::read_images_from_dir(tile_dir)?;

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

        if tiles.len() < grid_size.0 as usize * grid_size.1 as usize {
            return Err(format!(
                "Not enough tiles: {} for grid size: {}x{}",
                tiles.len(),
                grid_size.0,
                grid_size.1
            )
            .into());
        }

        Ok(Self {
            master,
            tiles,
            grid_size,
        })
    }

    /// Compute the (flat) distance matrix between the tiles and the master cells, using the
    /// [`NormL1`] metric
    ///
    /// To use a different distance metric, use the [`Mosaic::distance_matrix_with_metric`] method.
    ///
    /// The row index is the cell index and the column index is the tile index.
    pub fn distance_matrix(&self) -> Vec<i64> {
        self.distance_matrix_with_metric(Box::new(NormL1))
    }

    /// Compute the (flat) distance matrix between the tiles and the master cells using the provided
    /// `metric` which should implement the [`Metric`] trait
    ///
    /// The row index is the cell index and the column index is the tile index.
    pub fn distance_matrix_with_metric(&self, metric: Box<dyn Metric>) -> Vec<i64> {
        info!("Starting distance matrix computation...");
        let start_time = time::Instant::now();

        let d_matrix = utils::iter_or_par_iter!(self.master.cells)
            .flat_map(|cell| {
                utils::iter_or_par_iter!(self.tiles).map(|tile| metric.compute(tile, cell))
            })
            .collect();
        info!("Completed in {:?}", start_time.elapsed());
        d_matrix
    }

    /// Compute the tile to material cell assignments using the
    /// [pathfinding::kuhn_munkres](pathfinding::kuhn_munkres::kuhn_munkres_min) algorithm.
    pub fn build(&self, distance_matrix: Vec<i64>) -> Result<RgbImage, Error> {
        // use the hungarian algorithm to find the best tile to cell assignments
        let weights = pathfinding::matrix::Matrix::from_vec(
            self.master.cells.len(),
            self.tiles.len(),
            distance_matrix,
        )?;
        // the indice in assignments is the tile index
        // The value at the index is the index of the cell where is should be assigned
        info!("Solving the assignment problem...");
        let start_time = time::Instant::now();
        let (_, assignments) = pathfinding::kuhn_munkres::kuhn_munkres_min(&weights);
        info!("Completed in {:?}", start_time.elapsed());

        let (grid_width, grid_height) = self.grid_size;
        let (cell_width, cell_height) = self.master.cell_size;

        let mut mosaic_img = RgbImage::new(self.master.img.width(), self.master.img.height());
        for (cell_idx, tile_idx) in assignments.into_iter().enumerate() {
            // let cell = self.master.cells.get(*cell_idx).unwrap();
            let tile = self.tiles.get(tile_idx).unwrap();
            mosaic_img.copy_from(
                tile,
                (cell_idx as u32 % grid_width) * cell_width,
                (cell_idx as u32 / grid_height) * cell_height,
            )?;
        }
        Ok(mosaic_img)
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
        // tiles are 64x64
        test_dir().join("tiles/")
    }

    fn test_faces_dir() -> PathBuf {
        // from the UTKfaces dataset 1000 20x20 images of faces
        test_dir().join("faces/")
    }

    #[test]
    fn test_mosaic_creation_from_valid_data() {
        let grid_size = (4, 4);
        let mosaic = Mosaic::from_file_and_dir(test_master_img(), test_tile_dir(), grid_size);
        // Check if the mosaic was created successfully
        assert!(mosaic.is_ok());
        let mosaic = mosaic.unwrap();
        // Check that the master image has the expected dimensions
        assert_eq!(mosaic.master.img.width(), 256);
        assert_eq!(mosaic.master.img.height(), 256);
        // Check that the number of tiles matches the number of grid cells
        assert!(mosaic.tiles.len() >= mosaic.master.cells.len());
        // Make sure the tiles have the same size as the master cells
        assert!(mosaic
            .tiles
            .iter()
            .all(|tile| tile.dimensions() == mosaic.master.cell_size));
    }

    #[test]
    fn test_mosaic_creation_with_mismatched_tile_sizes() {
        // 5x5 grid which will not work with a 256x256 master image and 64x64 tiles
        let grid_size = (5, 5);
        // Attempt to create a mosaic and expect an error due to tile size mismatch
        let mosaic = Mosaic::from_file_and_dir(test_master_img(), test_tile_dir(), grid_size);
        assert!(mosaic.is_err());
    }

    #[test]
    fn test_invalid_master_file_path() {
        let grid_size = (4, 4);
        let mosaic = Mosaic::from_file_and_dir("invalid/master.png", test_tile_dir(), grid_size);
        assert!(mosaic.is_err());
    }

    #[test]
    fn test_invalid_tile_directory() {
        let grid_size = (4, 4);
        let mosaic = Mosaic::from_file_and_dir(test_master_img(), "invalid/tile_dir", grid_size);
        assert!(mosaic.is_err());
    }

    #[test]
    fn test_not_enough_tiles() {
        let master_img = image::open(test_master_img()).unwrap().to_rgb8();
        // use a small master image a tile just for testing
        let tiles = vec![image::imageops::resize(
            &master_img,
            64,
            64,
            image::imageops::FilterType::Nearest,
        )];
        let mosaic = Mosaic::from_images(master_img, tiles, (4, 4));
        assert!(mosaic.is_err());
    }

    #[test]
    fn test_distance_matrix() {
        let master_img = image::open(test_master_img()).unwrap().to_rgb8();
        let tiles = utils::read_images_from_dir(test_tile_dir()).unwrap();
        let mosaic = Mosaic::from_images(master_img, tiles, (4, 4)).unwrap();
        let distance_matrix = mosaic.distance_matrix();
        assert_eq!(
            distance_matrix.len(),
            mosaic.master.cells.len() * mosaic.tiles.len()
        );
    }

    #[test]
    fn test_mosaic_build() {
        let master_img = image::imageops::resize(
            &image::open(test_master_img()).unwrap().to_rgb8(),
            240,
            240,
            image::imageops::FilterType::Nearest,
        );

        let tiles_imgs = utils::read_images_from_dir(test_faces_dir()).unwrap();
        let result = Mosaic::from_images(master_img, tiles_imgs, (12, 12));
        assert!(result.is_ok());
        let mosaic = result.unwrap();

        assert_eq!(mosaic.master.cells.len(), 144);
        let d_matrix = mosaic.distance_matrix();
        let result = mosaic.build(d_matrix);
        assert!(result.is_ok());
        let mosaic_img = result.unwrap();
        assert_eq!(mosaic_img.width(), 240);
        assert_eq!(mosaic_img.height(), 240);

        let expected_path = test_dir().join("mosaic.png");
        if std::env::var("PHOMO_UPDATE_EXPECTED").is_ok() {
            mosaic_img.save(&expected_path).unwrap();
        }
        let expected_img = image::open(expected_path).unwrap().to_rgb8();
        assert_eq!(expected_img, mosaic_img);
    }
}
