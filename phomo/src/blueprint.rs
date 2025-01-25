use image::{GenericImage, RgbImage};
use log::info;
use serde::{Deserialize, Serialize};

use crate::error::{MosaicError, PhomoError};
use crate::solvers::hungarian::Hungarian;
use crate::solvers::{Solve, SolverConfig};
use crate::{DistanceMatrix, Mosaic};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Blueprint {
    pub cells: Vec<Cell>,
    pub cell_width: u32,
    pub cell_height: u32,
    pub grid_width: u32,
    pub grid_height: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Cell {
    pub tile_index: usize,
    pub x: u32,
    pub y: u32,
}

/// A serializable struct which represents a [crate::mosaic::Mosaic] that has yet to be rendered.
impl Blueprint {
    /// Render the [Blueprint].
    ///
    /// Errors:
    /// - [`PhomoError::MosaicError`]: An error occurred while rendering the mosaic.
    /// - [`PhomoError::ImageError`]: An error occurred while copying the tiles to the mosaic image.
    pub fn render(
        &self,
        master_img: &RgbImage,
        tiles: &[RgbImage],
    ) -> Result<RgbImage, PhomoError> {
        let mut mosaic_img = RgbImage::new(master_img.width(), master_img.height());
        info!(
            "Building mosaic, size: {}x{}, cell size: {}x{}, grid size: {}x{}",
            mosaic_img.width(),
            mosaic_img.height(),
            self.cell_width,
            self.cell_height,
            self.grid_width,
            self.grid_height
        );

        for cell in self.cells.iter() {
            let tile = tiles
                .get(cell.tile_index)
                .ok_or(MosaicError::InvalidTileIndex(cell.tile_index))?;
            mosaic_img.copy_from(tile, cell.x, cell.y)?;
        }
        Ok(mosaic_img)
    }
}

impl Mosaic {
    /// Compute the tile to master cell assignments, and construct a [`Blueprint`] of the mosaic
    /// image.
    ///
    /// # Errors
    /// - [`PhomoError::MosaicError`]: An error occurred while building the mosaic blueprint.
    /// - [`PhomoError::SolverError`]: An error occurred while solving the tile to cell assignments.
    pub fn build_blueprint(
        &self,
        distance_matrix: DistanceMatrix,
        config: SolverConfig,
    ) -> Result<Blueprint, PhomoError> {
        let solver = Hungarian::new(config);
        self.build_blueprint_with_solver(distance_matrix, solver)
    }

    /// Compute the tile to master cell assignments using the provided solver algorithm, and
    /// construct a [`Blueprint`] of the mosaic image.
    ///
    /// # Arguments:
    /// - `distance_matrix`: The distance matrix between the master image and the tiles.
    /// - `solver`: The solver algorithm to use for the assignment problem. See [`phomo::solvers`](crate::solvers) for structs
    ///     which implement this trait.
    ///
    /// # Errors
    /// - [`PhomoError::MosaicError`]: An error occurred while building the mosaic.
    /// - [`PhomoError::SolverError`]: An error occurred while solving the tile to cell assignments.
    pub fn build_blueprint_with_solver<S: Solve>(
        &self,
        distance_matrix: DistanceMatrix,
        mut solver: S,
    ) -> Result<Blueprint, PhomoError> {
        self.check_distance_matrix(&distance_matrix)?;

        let assignments = distance_matrix.assignments(&mut solver)?;
        let (grid_width, grid_height) = self.grid_size;
        let (cell_width, cell_height) = self.master.cell_size;
        info!(
            "Building mosaic blueprint, size: {}x{}, cell size: {}x{}, grid size: {}x{}",
            cell_width * grid_width,
            cell_height * grid_height,
            cell_width,
            cell_height,
            grid_width,
            grid_height
        );

        let cells = assignments
            .into_iter()
            .enumerate()
            .map(|(cell_idx, tile_idx)| {
                let x = (cell_idx as u32 % grid_width) * cell_width;
                let y = (cell_idx as u32 / grid_width) * cell_height;
                Cell {
                    tile_index: tile_idx % self.tiles.len(),
                    x,
                    y,
                }
            })
            .collect::<Vec<_>>();

        Ok(Blueprint {
            cells,
            cell_width,
            cell_height,
            grid_width,
            grid_height,
        })
    }
}
