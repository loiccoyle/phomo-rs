use crate::solvers::error::SolverError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PhomoError {
    #[error("Mosaic error: {0}")]
    MosaicError(#[from] MosaicError),

    #[error("Master error: {0}")]
    MasterError(#[from] MasterError),

    #[error("Solver error: {0}")]
    SolverError(#[from] SolverError),

    #[error("Distance matrix error: {0}")]
    DistanceMatrixError(#[from] DistanceMatrixError),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
}

#[derive(Debug, Error)]
pub enum MasterError {
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Invalid grid shape {grid_size:?}, the master dimensions {master_dimensions:?} are not divisible by the cell size {cell_size:?}")]
    GridSizeMismatch {
        grid_size: (u32, u32),
        master_dimensions: (u32, u32),
        cell_size: (u32, u32),
    },
}

#[derive(Debug, Error)]
pub enum MosaicError {
    #[error("Distance matrix size mismatch: expected {expected:?}, but found {found:?}")]
    DistanceMatrixSizeMismatch {
        expected: (usize, usize),
        found: (usize, usize),
    },
    #[error("Invalid assignments length: expected {expected:?}, but found {found:?}")]
    InvalidAssignmentsLength { expected: usize, found: usize },
    #[error("Tile size mismatch: expected {expected:?}, but found {found:?}")]
    TileSizeMismatch {
        expected: (u32, u32),
        found: (u32, u32),
    },
    #[error("Not enough tiles provided: {provided} tiles for {required} required with max occurrences of {max_occurrences}")]
    InsufficientTiles {
        provided: usize,
        required: usize,
        max_occurrences: usize,
    },

    #[error("Invalid tile index: {0}")]
    InvalidTileIndex(usize),
    #[error("{0}")]
    Custom(String),
}

#[derive(Debug, Error)]
pub enum DistanceMatrixError {
    #[error("The length of the data vector does not match the specified dimensions")]
    WrongLength,

    #[error("The number of columns must be greater than zero")]
    EmptyCol,

    #[error("The number of rows must be greater than zero")]
    EmptyRow,
}
