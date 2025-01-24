/// A module which contains structs which implement the [`Solve`](crate::solvers::Solve) trait. These structs are used to
/// solve the assignment problem.
use crate::{error::PhomoError, DistanceMatrix};

pub mod error;
pub mod greedy;
pub mod hungarian;

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

pub trait Solve {
    fn configure(&mut self, config: SolverConfig);
    fn solve(&mut self, distance_matrix: &DistanceMatrix) -> Result<Vec<usize>, PhomoError>;
}
