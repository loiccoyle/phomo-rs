use std::path::Path;
#[cfg(not(target_family = "wasm"))]
use std::time;

extern crate image;
use image::{GenericImage, RgbImage};
use log::info;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::distance_matrix::DistanceMatrix;
use crate::error::{MosaicError, PhomoError};
use crate::macros;
use crate::master::Master;
use crate::metrics::{norm_l1, MetricFn};
use crate::solvers::{hungarian::Hungarian, Solve, SolverConfig};
use crate::utils;

#[derive(Debug, Clone)]
pub struct Mosaic {
    /// The [`Master`] image to reconstruct.
    pub master: Master,
    /// The tile images to use to reconstruct the [`Master`] image. The tile images should be the
    /// same size as the [`Master::cell_size`]. There should also be at least `Master::cells.len()`
    /// tiles.
    pub tiles: Vec<RgbImage>,
    /// The number of cells horizontally and vertically in the mosaic.
    pub grid_size: (u32, u32),
}

/// Represents a photo mosaic.
impl Mosaic {
    /// Construct a [`Mosaic`] from a master image file and a directory of tile images.
    ///
    /// # Arguments
    /// - `master_file`: The path to the master image file.
    /// - `tile_dir`: The path to the directory containing the tile images.
    /// - `grid_size`: The grid size of the mosaic, the number of cells horizontally and vertically.
    ///
    /// # Errors
    /// - [`PhomoError::ImageError`]: An error occurred while reading the master image.
    /// - [`PhomoError::MasterError`]: An error occurred constructing the [`Master`] from the master image.
    /// - [`PhomoError::IoError`]: An error occurred while reading the tile images.
    pub fn from_file_and_dir<P: AsRef<Path>, Q: AsRef<Path>>(
        master_file: P,
        tile_dir: Q,
        grid_size: (u32, u32),
    ) -> Result<Self, PhomoError> {
        let master_img = image::open(master_file)?.to_rgb8();
        info!("Loading tiles");
        let tiles = utils::read_images_from_dir(tile_dir)?;

        Self::from_images(master_img, tiles, grid_size)
    }

    /// Construct a [`Mosaic`] from [`RgbImage`] buffers of the master images and the tile
    /// images.
    ///
    /// # Arguments
    /// - `master_img`: The master image buffer.
    /// - `tiles`: The tile image buffers.
    /// - `grid_size`: The grid size of the mosaic, the number of cells horizontally and vertically.
    /// - `max_tile_occurrences`: The maximum number of times a tile can be repeated in the mosaic.
    ///     Should be greater than 0.
    ///
    /// # Errors
    /// - [`PhomoError::MasterError`]: An error occurred while constructing the [`Master`].
    /// - [`PhomoError::MosaicError`]: An error occurred while constructing the [`Mosaic`].
    pub fn from_images(
        master_img: RgbImage,
        tiles: Vec<RgbImage>,
        grid_size: (u32, u32),
    ) -> Result<Self, PhomoError> {
        let master = Master::from_image(master_img, grid_size)?;
        Self::new(master, tiles, grid_size)
    }

    /// Create a new [`Mosaic`] from the provided [`Master`] and tiles.
    ///
    /// # Arguments
    /// - `master`: The master.
    /// - `tiles`: The tile image buffers.
    /// - `grid_size`: The grid size of the mosaic, the number of cells horizontally and vertically.
    ///
    /// # Errors
    /// - [`PhomoError::MosaicError`]: An error occurred while constructing the [`Mosaic`].
    pub fn new(
        master: Master,
        tiles: Vec<RgbImage>,
        grid_size: (u32, u32),
    ) -> Result<Self, PhomoError> {
        if let Some(mismatched_tile) = tiles
            .iter()
            .find(|img| img.dimensions() != master.cell_size)
        {
            return Err(MosaicError::TileSizeMismatch {
                expected: master.cell_size,
                found: mismatched_tile.dimensions(),
            }
            .into());
        }

        Ok(Self {
            master,
            tiles,
            grid_size,
        })
    }

    /// Compute the (flat) distance matrix between the tiles and the master cells, using the
    /// [`norm_l1`] metric.
    ///
    /// To use a different distance metric, use the [`distance_matrix_with_metric`](Mosaic::distance_matrix_with_metric) method.
    ///
    /// The row index is the cell index and the column index is the tile index.
    pub fn distance_matrix(&self) -> DistanceMatrix {
        self.distance_matrix_with_metric(norm_l1)
    }

    /// Compute the (flat) distance matrix between the tiles and the master cells using the provided
    /// `metric` function, see [`phomo::metrics`](crate::metrics) for implemented distance metrics.
    ///
    /// The row index is the cell index and the column index is the tile index.
    pub fn distance_matrix_with_metric(&self, metric: MetricFn) -> DistanceMatrix {
        #[cfg(not(target_family = "wasm"))]
        info!("Starting distance matrix computation...");
        #[cfg(not(target_family = "wasm"))]
        let start_time = time::Instant::now();

        let d_matrix = macros::maybe_progress_bar!(
            macros::iter_or_par_iter!(self.master.cells),
            "Computing distance matrix",
            par
        )
        .flat_map(|cell| macros::iter_or_par_iter!(self.tiles).map(|tile| metric(tile, cell)))
        .collect();

        #[cfg(not(target_family = "wasm"))]
        info!("Completed in {:?}", start_time.elapsed());

        // We can construct the struct directly because we know the sizes should line up
        DistanceMatrix {
            rows: self.master.cells.len(),
            columns: self.tiles.len(),
            data: d_matrix,
        }
    }

    pub(crate) fn check_distance_matrix(
        &self,
        distance_matrix: &DistanceMatrix,
    ) -> Result<(), PhomoError> {
        if distance_matrix.rows != self.master.cells.len()
            || distance_matrix.columns < self.tiles.len()
        {
            return Err(MosaicError::DistanceMatrixSizeMismatch {
                expected: (self.master.cells.len(), self.tiles.len()),
                found: (distance_matrix.rows, distance_matrix.columns),
            }
            .into());
        }
        Ok(())
    }

    /// Render the photo mosaic image using the provided tile assignments.
    ///
    /// # Arguments
    /// - `assignments`: The tile index assigned to each master cell.
    ///    The length of the assignments should be equal to the number of master cells.
    ///    The tile index should be less than the number of tiles.
    ///
    /// # Errors
    /// - [`PhomoError::MosaicError`]: An error occurred while rendering the mosaic.
    /// - [`PhomoError::ImageError`]: An error occurred while copying the tiles to the mosaic image.
    pub fn render(&self, assignments: Vec<usize>) -> Result<RgbImage, PhomoError> {
        if assignments.len() != self.master.cells.len() {
            return Err(MosaicError::InvalidAssignmentsLength {
                expected: self.master.cells.len(),
                found: assignments.len(),
            }
            .into());
        }

        let (grid_width, _) = self.grid_size;
        let (cell_width, cell_height) = self.master.cell_size;
        let mut mosaic_img = RgbImage::new(self.master.img.width(), self.master.img.height());
        for (cell_idx, tile_idx) in assignments.into_iter().enumerate() {
            let x = (cell_idx as u32 % grid_width) * cell_width;
            let y = (cell_idx as u32 / grid_width) * cell_height;
            let tile = self
                .tiles
                .get(tile_idx % self.tiles.len())
                .ok_or(MosaicError::InvalidTileIndex(tile_idx))?;
            mosaic_img.copy_from(tile, x, y)?;
        }
        Ok(mosaic_img)
    }

    /// Compute the tile to master cell assignments using the [Hungarian] solver
    /// algorithm, and build the photo mosaic image.
    ///
    /// # Errors
    /// - [`PhomoError::MosaicError`]: An error occurred while building the mosaic.
    /// - [`PhomoError::SolverError`]: An error occurred while solving the tile to cell assignments.
    pub fn build(
        &self,
        distance_matrix: DistanceMatrix,
        config: SolverConfig,
    ) -> Result<RgbImage, PhomoError> {
        let solver = Hungarian::new(config);
        self.build_with_solver(distance_matrix, solver)
    }

    /// Compute the tile to master cell assignments using the provided solver algorithm, and build
    /// the photo mosaic image.
    ///
    /// # Errors
    /// - [`PhomoError::MosaicError`]: An error occurred while building the mosaic.
    /// - [`PhomoError::SolverError`]: An error occurred while solving the tile to cell assignments.
    pub fn build_with_solver<S: Solve>(
        &self,
        distance_matrix: DistanceMatrix,
        mut solver: S,
    ) -> Result<RgbImage, PhomoError> {
        self.check_distance_matrix(&distance_matrix)?;
        let assignments = distance_matrix.assignments(&mut solver)?;
        self.render(assignments)
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
    fn test_distance_matrix() {
        let master_img = image::open(test_master_img()).unwrap().to_rgb8();
        let tiles = utils::read_images_from_dir(test_tile_dir()).unwrap();
        let mosaic = Mosaic::from_images(master_img, tiles, (4, 4)).unwrap();
        let distance_matrix = mosaic.distance_matrix();
        assert_eq!(
            distance_matrix.data.len(),
            mosaic.master.cells.len() * mosaic.tiles.len()
        );
    }

    #[test]
    fn test_too_few_tiles() {
        let master_img = image::open(test_master_img()).unwrap().to_rgb8();
        let tiles = utils::read_images_from_dir(test_tile_dir())
            .unwrap()
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        let mosaic = Mosaic::from_images(master_img, tiles, (4, 4)).unwrap();
        let d_matrix = mosaic.distance_matrix();
        let result = mosaic.build(d_matrix, SolverConfig::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_too_few_tiles_unless_repeats() {
        let master_img = image::open(test_master_img()).unwrap().to_rgb8();
        let tiles = utils::read_images_from_dir(test_tile_dir())
            .unwrap()
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        let mosaic = Mosaic::from_images(master_img, tiles, (4, 4)).unwrap();
        let d_matrix = mosaic.distance_matrix();
        let result = mosaic.build(
            d_matrix,
            SolverConfig {
                max_tile_occurrences: 8,
            },
        );
        assert!(result.is_ok());
    }
}
