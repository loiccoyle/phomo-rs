use std::fmt;

use crate::error::LsapError;
use crate::lsap;

#[derive(Debug)]
pub enum DistanceMatrixError {
    WrongLength,
    EmptyRow,
    EmptyCol,
}

impl fmt::Display for DistanceMatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistanceMatrixError::WrongLength => write!(
                f,
                "The length of the data vector does not match the specified dimensions"
            ),
            DistanceMatrixError::EmptyCol => {
                write!(f, "The number of columns must be greater than zero")
            }
            DistanceMatrixError::EmptyRow => {
                write!(f, "The number of rows must be greater than zero")
            }
        }
    }
}

impl std::error::Error for DistanceMatrixError {}

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
    pub fn new(rows: usize, columns: usize, data: Vec<i64>) -> Result<Self, DistanceMatrixError> {
        if rows * columns != data.len() {
            return Err(DistanceMatrixError::WrongLength);
        }
        if rows == 0 {
            return Err(DistanceMatrixError::EmptyRow);
        }
        if columns == 0 {
            return Err(DistanceMatrixError::EmptyCol);
        }
        Ok(Self {
            rows,
            columns,
            data,
        })
    }
}

impl DistanceMatrix {
    /// Solve the linear sum assignment problem using the Kuhn-Munkres algorithm.
    pub fn assignments(&self) -> Result<Vec<usize>, LsapError> {
        lsap::solve(self)
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
