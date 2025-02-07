/// A module which contains structs which implement the [`Solve`](crate::solvers::Solve) trait. These structs are used to
/// solve the assignment problem.
use crate::{error::PhomoError, DistanceMatrix};

pub mod auction;
pub mod error;
pub mod greedy;
pub mod hungarian;

/// Common configuration for the solvers.
#[derive(Debug, Clone)]
pub struct SolverConfig {
    pub max_tile_occurrences: usize,
}

impl Default for SolverConfig {
    fn default() -> Self {
        SolverConfig {
            max_tile_occurrences: 1,
        }
    }
}

/// The [`Solve`] trait is implemented by structs which solve the tile to cell assignment problem.
pub trait Solve {
    /// Solve the assignment problem using the solver.
    ///
    /// # Arguments
    /// - `distance_matrix`: The distance matrix.
    ///
    /// # Errors
    /// - [`PhomoError::SolverError``]: An error occurred while solving the assignment problem.
    fn solve(&mut self, distance_matrix: &DistanceMatrix) -> Result<Vec<usize>, PhomoError>;
}
