use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolverError {
    #[error("Lsap error: {0}")]
    HungarianError(#[from] HungarianError),

    #[error(
        "Too few columns in the distance matrix, expected at least {rows}, but found {columns}"
    )]
    TooFewColumns { rows: usize, columns: usize },
}

#[derive(Debug, Error)]
pub enum HungarianError {
    #[error("Infeasible")]
    Infeasible,
}
