#[cfg(not(target_family = "wasm"))]
use log::info;
#[cfg(not(target_family = "wasm"))]
use std::time;

use crate::error::DistanceMatrixError;
use crate::error::PhomoError;
use crate::solvers::Solve;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct DistanceMatrix {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<i64>,
}

impl DistanceMatrix {
    /// Create a new [DistanceMatrix] which is tiled `n` times horizontally.
    pub(crate) fn tile(&self, n: usize) -> DistanceMatrix {
        DistanceMatrix {
            rows: self.rows,
            columns: self.columns * n,
            data: self
                .data
                .chunks(self.columns)
                .flat_map(|row| row.repeat(n))
                .collect(),
        }
    }
}

/// Helper struct to handle the distance matrix and allow for repeated tiles.
impl DistanceMatrix {
    pub fn new(rows: usize, columns: usize, data: Vec<i64>) -> Result<Self, PhomoError> {
        if rows * columns != data.len() {
            return Err(DistanceMatrixError::WrongLength.into());
        }
        if rows == 0 {
            return Err(DistanceMatrixError::EmptyRow.into());
        }
        if columns == 0 {
            return Err(DistanceMatrixError::EmptyCol.into());
        }
        Ok(Self {
            rows,
            columns,
            data,
        })
    }

    pub fn get(&self, row: usize, col: usize) -> i64 {
        self.data[row * self.columns + col]
    }
}

impl DistanceMatrix {
    /// Solve the linear sum assignment problem using the provided `solver`.
    ///
    /// # Arguments
    /// - `solver`: The solver to use to solve the assignment problem. See [`phomo::solvers`](crate::solvers)
    ///     for structs which implement this trait.
    ///
    /// # Errors
    /// - [`PhomoError::SolverError`]: An error occurred while solving the assignment problem.
    pub fn assignments<S: Solve>(&self, solver: &mut S) -> Result<Vec<usize>, PhomoError> {
        #[cfg(not(target_family = "wasm"))]
        info!("Computing assignmnent...");
        #[cfg(not(target_family = "wasm"))]
        let start_time = time::Instant::now();

        let out = solver.solve(self);

        #[cfg(not(target_family = "wasm"))]
        info!("Completed in {:?}", start_time.elapsed());

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let d_matrix = DistanceMatrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        assert!(d_matrix.is_ok());
    }

    #[test]
    fn test_new_bad_size() {
        let d_matrix = DistanceMatrix::new(2, 3, vec![1, 2, 3, 4]);
        assert!(d_matrix.is_err());
    }

    #[test]
    fn test_tile() {
        let d_matrix = DistanceMatrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let d_matrix_repeat = d_matrix.tile(2);
        assert!(d_matrix_repeat.data == vec![1, 2, 3, 1, 2, 3, 4, 5, 6, 4, 5, 6]);
    }
}
