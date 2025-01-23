use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use image::{GenericImage, RgbImage};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{error::Error, DistanceMatrix, Mosaic};

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
    pub fn render(&self, master_img: &RgbImage, tiles: &[RgbImage]) -> Result<RgbImage, Error> {
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
            let tile = tiles.get(cell.tile_index).ok_or_else(|| {
                format!("Tile index {} out of range", cell.tile_index).to_string()
            })?;
            mosaic_img.copy_from(tile, cell.x, cell.y)?;
        }
        Ok(mosaic_img)
    }
}

impl Mosaic {
    /// Compute the tile to master cell assignments, and construct a [`Blueprint`] of the mosaic
    /// image.
    pub fn build_blueprint(&self, distance_matrix: DistanceMatrix) -> Result<Blueprint, Error> {
        if distance_matrix.rows != self.master.cells.len()
            || distance_matrix.columns < self.tiles.len()
        {
            return Err(
                "The distance matrix rows must match the number of master cells, and the number of columns must be greater than or equal to the number of tiles.".into(),
            );
        }

        let assignments = if self.max_tile_occurrences > 1 {
            distance_matrix.tile(self.max_tile_occurrences)
        } else {
            distance_matrix
        }
        .assignments();
        let (grid_width, grid_height) = self.grid_size;
        let (cell_width, cell_height) = self.master.cell_size;

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

    /// Compute the tile to master cell assignments using a greedy algorithm, and construct a [`Blueprint`] of the mosaic
    /// image.
    pub fn build_blueprint_greedy(
        &self,
        distance_matrix: DistanceMatrix,
    ) -> Result<Blueprint, Error> {
        if distance_matrix.rows != self.master.cells.len()
            || distance_matrix.columns < self.tiles.len()
        {
            return Err(
            "The distance matrix rows must match the number of master cells, and the number of columns must be greater than or equal to the number of tiles.".into(),
        );
        }

        let (grid_width, grid_height) = self.grid_size;
        let (cell_width, cell_height) = self.master.cell_size;

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

        let mut cells = Vec::with_capacity(self.master.cells.len());

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
            cells.push(Cell {
                tile_index: tile_idx,
                x,
                y,
            });

            filled_master_cells.insert(cell_idx);
            n_appearances[tile_idx] += 1;

            if n_appearances[tile_idx] == self.max_tile_occurrences {
                placed_tiles.insert(tile_idx);
            }
        }

        Ok(Blueprint {
            cells,
            cell_width,
            cell_height,
            grid_width,
            grid_height,
        })
    }
}
