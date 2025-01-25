use std::{cmp::Reverse, collections::BinaryHeap};

use crate::solvers::Solve;
use crate::solvers::SolverConfig;
use crate::DistanceMatrix;
use crate::{error::PhomoError, solvers::error::SolverError};

/// The Greedy struct represents the greedy algorithm solver.
#[derive(Debug, Default)]
pub struct Greedy {
    config: SolverConfig,
}

impl Greedy {
    /// Creates a new instance of the Greedy struct with the given configuration.
    ///
    /// # Arguments
    /// - `config`: The configuration for the solver.
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

        let mut n_appearances = vec![0; n_tiles];
        let mut heap = BinaryHeap::with_capacity(n_cells);

        // Initialize the heap with the best tile for each cell
        for row_idx in 0..n_cells {
            let mut best_tile = 0;
            let mut best_distance = distance_matrix.get(row_idx, best_tile);
            for col_idx in 1..n_tiles {
                let distance = distance_matrix.get(row_idx, col_idx);
                if distance < best_distance {
                    best_distance = distance;
                    best_tile = col_idx;
                }
            }
            heap.push(Reverse((best_distance, row_idx, best_tile)));
        }

        let mut assignments = vec![0; n_cells];
        let mut filled_count = 0;

        while let Some(Reverse((_, cell_idx, tile_idx))) = heap.pop() {
            if filled_count == n_cells {
                break;
            }

            // Check if the tile is still available
            if n_appearances[tile_idx] < self.config.max_tile_occurrences {
                assignments[cell_idx] = tile_idx;
                n_appearances[tile_idx] += 1;
                filled_count += 1;
            } else {
                // Tile is no longer available, find the next best tile for this cell
                let mut next_best_tile = 0;
                let mut next_best_distance = i64::MAX;
                for (tile_idx, n) in n_appearances.iter().enumerate() {
                    if *n < self.config.max_tile_occurrences {
                        let distance = distance_matrix.get(cell_idx, tile_idx);
                        if distance < next_best_distance {
                            next_best_distance = distance;
                            next_best_tile = tile_idx;
                        }
                    }
                }
                heap.push(Reverse((next_best_distance, cell_idx, next_best_tile)));
            }
        }

        Ok(assignments)
    }
}
