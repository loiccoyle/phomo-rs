use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use crate::solvers::Solve;
use crate::solvers::SolverConfig;
use crate::DistanceMatrix;
use crate::{error::PhomoError, solvers::error::SolverError};

#[derive(Debug, Default)]
pub struct Greedy {
    config: SolverConfig,
}

impl Greedy {
    pub fn new(config: SolverConfig) -> Self {
        Self { config }
    }
}

impl Solve for Greedy {
    fn configure(&mut self, config: SolverConfig) {
        self.config = config;
    }

    fn solve(&mut self, distance_matrix: &DistanceMatrix) -> Result<Vec<usize>, PhomoError> {
        if distance_matrix.columns < distance_matrix.rows * self.config.max_tile_occurrences {
            return Err(SolverError::TooFewColumns {
                rows: distance_matrix.rows,
                columns: distance_matrix.columns,
            }
            .into());
        }

        let n_cells = distance_matrix.rows;
        let n_tiles = distance_matrix.columns;

        let mut filled_master_cells = HashSet::with_capacity(n_cells);
        let mut placed_tiles = HashSet::with_capacity(n_tiles);
        let mut n_appearances = vec![0; n_tiles];
        let mut heap = BinaryHeap::with_capacity(n_cells * n_tiles);

        for row_idx in 0..n_cells {
            for col_idx in 0..n_tiles {
                let distance = distance_matrix.data[row_idx * n_tiles + col_idx];
                heap.push(Reverse((distance, row_idx, col_idx)));
            }
        }

        let mut assignments = vec![0; n_cells];

        while let Some(Reverse((_, cell_idx, tile_idx))) = heap.pop() {
            if filled_master_cells.len() == n_cells {
                // stop early if all the master cells have been filled
                break;
            }

            if filled_master_cells.contains(&cell_idx) || placed_tiles.contains(&tile_idx) {
                continue;
            }

            assignments[cell_idx] = tile_idx;

            filled_master_cells.insert(cell_idx);
            n_appearances[tile_idx] += 1;

            if n_appearances[tile_idx] == self.config.max_tile_occurrences {
                placed_tiles.insert(tile_idx);
            }
        }
        Ok(assignments)
    }
}
