#[cfg(not(target_family = "wasm"))]
use log::info;
#[cfg(not(target_family = "wasm"))]
use std::time;

extern crate pathfinding;
use pathfinding::matrix::Matrix;
use pathfinding::matrix::MatrixFormatError;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct DistanceMatrix<C> {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<C>,
}

impl<C: std::clone::Clone> From<Matrix<C>> for DistanceMatrix<C> {
    fn from(value: Matrix<C>) -> Self {
        DistanceMatrix {
            rows: value.rows,
            columns: value.columns,
            data: value.values().cloned().collect(),
        }
    }
}

impl<C> From<DistanceMatrix<C>> for Matrix<C> {
    fn from(val: DistanceMatrix<C>) -> Self {
        Matrix::from_vec(val.rows, val.columns, val.data).unwrap()
    }
}

impl<C: std::marker::Copy> DistanceMatrix<C> {
    /// Create a new [DistanceMatrix] which is tiled `n` times horizontally.
    pub(crate) fn tile(&self, n: usize) -> DistanceMatrix<C> {
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
impl<C> DistanceMatrix<C> {
    pub fn new(rows: usize, columns: usize, data: Vec<C>) -> Result<Self, MatrixFormatError> {
        if rows * columns != data.len() {
            return Err(MatrixFormatError::WrongLength);
        }
        if rows != 0 && columns == 0 {
            return Err(MatrixFormatError::EmptyRow);
        }
        Ok(Self {
            rows,
            columns,
            data,
        })
    }
}

impl<
        C: std::clone::Clone
            + std::marker::Copy
            + pathfinding::num_traits::Bounded
            + pathfinding::num_traits::Signed
            + std::cmp::Ord
            + std::iter::Sum,
    > DistanceMatrix<C>
{
    /// Solve the assignment problem using the Kuhn-Munkres algorithm.
    pub fn assignments(&self) -> Vec<usize> {
        let weights: Matrix<C> = self.clone().into();
        // the indice in assignments is the tile index
        // The value at the index is the index of the cell where is should be assigned
        #[cfg(not(target_family = "wasm"))]
        info!("Solving the assignment problem...");
        #[cfg(not(target_family = "wasm"))]
        let start_time = time::Instant::now();
        let (_, assignments) = pathfinding::kuhn_munkres::kuhn_munkres_min(&weights);
        #[cfg(not(target_family = "wasm"))]
        info!("Completed in {:?}", start_time.elapsed());
        assignments
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
