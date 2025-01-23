use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::path::Path;
#[cfg(not(target_family = "wasm"))]
use std::time;

extern crate image;
use image::{GenericImage, RgbImage};
use log::info;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::distance_matrix::DistanceMatrix;
use crate::error::MosaicError;
use crate::macros;
use crate::master::Master;
use crate::metrics::{norm_l1, MetricFn};
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
    /// Tile can be repeated up to `max_tile_occurrences` times in the mosaic. Should be greater
    /// than 0.
    pub max_tile_occurrences: usize,
}

/// Represents a photo mosaic.
impl Mosaic {
    /// Construct a [`Mosaic`] from a master image file and a directory of tile images.
    ///
    /// # Arguments
    /// - `master_file`: The path to the master image file.
    /// - `tile_dir`: The path to the directory containing the tile images.
    /// - `grid_size`: The grid size of the mosaic, the number of cells horizontally and vertically.
    /// - `max_tile_occurrences`: The maximum number of times a tile can be repeated in the mosaic.
    ///     Should be greater than 0.
    ///
    /// # Errors
    /// - [MosaicError::ImageError]: An error occurred while reading the master image.
    /// - [MosaicError::IoError]: An error occurred while reading the tile images.
    pub fn from_file_and_dir<P: AsRef<Path>, Q: AsRef<Path>>(
        master_file: P,
        tile_dir: Q,
        grid_size: (u32, u32),
        max_tile_occurrences: usize,
    ) -> Result<Self, MosaicError> {
        let master_img = image::open(master_file)?.to_rgb8();
        info!("Loading tiles");
        let tiles = utils::read_images_from_dir(tile_dir)?;

        Self::from_images(master_img, tiles, grid_size, max_tile_occurrences)
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
    /// - [MosaicError::MasterError]: an error occurred while reading the master image or the tile images.
    /// - [MosaicError::Custom]: max_tile_occurrences must be greater than 0.
    /// - [MosaicError::TileSizeMismatch]: The tile images were not all the same size as the grid cells.
    /// - [MosaicError::InsufficientTiles]: Not enough tile images were provided for the `grid_size`.
    pub fn from_images(
        master_img: RgbImage,
        tiles: Vec<RgbImage>,
        grid_size: (u32, u32),
        max_tile_occurrences: usize,
    ) -> Result<Self, MosaicError> {
        let master = Master::from_image(master_img, grid_size)?;
        Self::new(master, tiles, grid_size, max_tile_occurrences)
    }

    /// Create a new [`Mosaic`] from the provided [`Master`] and tiles.
    ///
    /// # Arguments
    /// - `master`: The master.
    /// - `tiles`: The tile image buffers.
    /// - `grid_size`: The grid size of the mosaic, the number of cells horizontally and vertically.
    /// - `max_tile_occurrences`: The maximum number of times a tile can be repeated in the mosaic.
    ///     Should be greater than 0.
    ///
    /// # Errors
    /// - [MosaicError::Custom]: `max_tile_occurrences` must be greater than 0.
    /// - [MosaicError::TileSizeMismatch]: The tile images were not all the same size as the grid cells.
    /// - [MosaicError::InsufficientTiles]: Not enough tile images were provided for the `grid_size`.
    pub fn new(
        master: Master,
        tiles: Vec<RgbImage>,
        grid_size: (u32, u32),
        max_tile_occurrences: usize,
    ) -> Result<Self, MosaicError> {
        if max_tile_occurrences == 0 {
            return Err(MosaicError::Custom(
                "max_tile_occurrences must be greater than 0".to_string(),
            ));
        }

        if let Some(mismatched_tile) = tiles
            .iter()
            .find(|img| img.dimensions() != master.cell_size)
        {
            return Err(MosaicError::TileSizeMismatch {
                expected: master.cell_size,
                found: mismatched_tile.dimensions(),
            });
        }

        if tiles.len() * max_tile_occurrences < grid_size.0 as usize * grid_size.1 as usize {
            return Err(MosaicError::InsufficientTiles {
                provided: tiles.len(),
                required: grid_size.0 as usize * grid_size.1 as usize,
                max_occurrences: max_tile_occurrences,
            });
        }

        Ok(Self {
            master,
            tiles,
            grid_size,
            max_tile_occurrences,
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
    ) -> Result<(), MosaicError> {
        if distance_matrix.rows != self.master.cells.len()
            || distance_matrix.columns < self.tiles.len()
        {
            return Err(MosaicError::DistanceMatrixSizeMismatch {
                expected: (self.master.cells.len(), self.tiles.len()),
                found: (distance_matrix.rows, distance_matrix.columns),
            });
        }
        Ok(())
    }

    /// Compute the tile to master cell assignments using the Kuhn-Munkres (Hungarian)
    /// algorithm, and build the photo mosaic image.
    ///
    /// # Errors
    /// - [MosaicError::DistanceMatrixSizeMismatch]: The size of the distance matrix does not match
    ///     the number of master cells and the number of tiles.
    /// - [MosaicError::LsapError]: The distance matrix is not solvable.
    /// - [MosaicError::InvalidTileIndex]: An invalid tile index was encountered when building the
    ///     image.
    /// - [MosaicError::ImageError]: An error occurred while copying the tile to the mosaic image.
    pub fn build(&self, distance_matrix: DistanceMatrix) -> Result<RgbImage, MosaicError> {
        self.check_distance_matrix(&distance_matrix)?;

        let assignments = if self.max_tile_occurrences > 1 {
            distance_matrix.tile(self.max_tile_occurrences)
        } else {
            distance_matrix
        }
        .assignments()?;

        let (grid_width, grid_height) = self.grid_size;
        let (cell_width, cell_height) = self.master.cell_size;

        let mut mosaic_img = RgbImage::new(self.master.img.width(), self.master.img.height());
        info!(
            "Building mosaic, size: {}x{}, cell size: {}x{}, grid size: {}x{}",
            mosaic_img.width(),
            mosaic_img.height(),
            cell_width,
            cell_height,
            grid_width,
            grid_height
        );
        for (cell_idx, tile_idx) in assignments.into_iter().enumerate() {
            let x = (cell_idx as u32 % grid_width) * cell_width;
            let y = (cell_idx as u32 / grid_width) * cell_height;
            let tile = self
                .tiles
                .get(tile_idx % self.tiles.len())
                .ok_or_else(|| MosaicError::InvalidTileIndex(tile_idx))?;
            mosaic_img.copy_from(tile, x, y)?;
        }
        Ok(mosaic_img)
    }

    /// Build the mosaic image using a greedy tile assignment algorithm. This leads to less
    /// accurate mosaics, but should be significantly faster, especially when the distance matrix
    /// is large.
    ///
    /// # Errors
    /// - [MosaicError::DistanceMatrixSizeMismatch]: The size of the distance matrix does not match
    ///     the number of master cells and the number of tiles.
    /// - [MosaicError::InvalidTileIndex]: An invalid tile index was encountered.
    /// - [MosaicError::ImageError]: An error occurred while copying the tile to the mosaic image.
    pub fn build_greedy(&self, distance_matrix: DistanceMatrix) -> Result<RgbImage, MosaicError> {
        self.check_distance_matrix(&distance_matrix)?;

        let (grid_width, grid_height) = self.grid_size;
        let (cell_width, cell_height) = self.master.cell_size;
        let mut mosaic_img = RgbImage::new(self.master.img.width(), self.master.img.height());
        info!(
            "Building mosaic, size: {}x{}, cell size: {}x{}, grid size: {}x{}",
            mosaic_img.width(),
            mosaic_img.height(),
            cell_width,
            cell_height,
            grid_width,
            grid_height
        );

        let mut filled_master_cells = HashSet::with_capacity(self.master.cells.len());
        let mut placed_tiles = HashSet::with_capacity(self.tiles.len());
        let mut n_appearances = vec![0; self.tiles.len()];
        let mut heap = BinaryHeap::with_capacity(distance_matrix.rows * distance_matrix.columns);

        // Populate the heap with (distance, row_idx, col_idx)
        for row_idx in 0..distance_matrix.rows {
            for col_idx in 0..distance_matrix.columns {
                let distance = distance_matrix.data[row_idx * distance_matrix.columns + col_idx];
                heap.push(Reverse((distance, row_idx, col_idx)));
            }
        }

        // Process elements in ascending order of distance
        while let Some(Reverse((_, cell_idx, tile_idx))) = heap.pop() {
            if filled_master_cells.len() == self.master.cells.len() {
                // stop early if all the master cells have been filled
                break;
            }

            if filled_master_cells.contains(&cell_idx) || placed_tiles.contains(&tile_idx) {
                continue;
            }

            let x = (cell_idx as u32 % grid_width) * cell_width;
            let y = (cell_idx as u32 / grid_width) * cell_height;

            let tile = self
                .tiles
                .get(tile_idx)
                .ok_or_else(|| MosaicError::InvalidTileIndex(tile_idx))?;
            mosaic_img.copy_from(tile, x, y)?;

            filled_master_cells.insert(cell_idx);
            n_appearances[tile_idx] += 1;

            if n_appearances[tile_idx] == self.max_tile_occurrences {
                placed_tiles.insert(tile_idx);
            }
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

    #[test]
    fn test_mosaic_creation_from_valid_data() {
        let grid_size = (4, 4);
        let mosaic = Mosaic::from_file_and_dir(test_master_img(), test_tile_dir(), grid_size, 1);
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
        let mosaic = Mosaic::from_file_and_dir(test_master_img(), test_tile_dir(), grid_size, 1);
        assert!(mosaic.is_err());
    }

    #[test]
    fn test_invalid_master_file_path() {
        let grid_size = (4, 4);
        let mosaic = Mosaic::from_file_and_dir("invalid/master.png", test_tile_dir(), grid_size, 1);
        assert!(mosaic.is_err());
    }

    #[test]
    fn test_invalid_tile_directory() {
        let grid_size = (4, 4);
        let mosaic = Mosaic::from_file_and_dir(test_master_img(), "invalid/tile_dir", grid_size, 1);
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
        let mosaic = Mosaic::from_images(master_img, tiles, (4, 4), 1);
        assert!(mosaic.is_err());
    }

    #[test]
    fn test_distance_matrix() {
        let master_img = image::open(test_master_img()).unwrap().to_rgb8();
        let tiles = utils::read_images_from_dir(test_tile_dir()).unwrap();
        let mosaic = Mosaic::from_images(master_img, tiles, (4, 4), 1).unwrap();
        let distance_matrix = mosaic.distance_matrix();
        assert_eq!(
            distance_matrix.data.len(),
            mosaic.master.cells.len() * mosaic.tiles.len()
        );
    }
}
