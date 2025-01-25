use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolverError {
    #[error("Lsap error: {0}")]
    HungarianError(#[from] HungarianError),
    #[error("Auction error: {0}")]
    AuctionError(#[from] AuctionError),

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

#[derive(Debug, Error)]
pub enum AuctionError {
    #[error("Unassigned agents.")]
    UnassignedAgents,
}
