use std::fmt;
use std::io;

#[derive(Debug)]
pub enum LsapError {
    Infeasible,
}

impl std::fmt::Display for LsapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LsapError::Infeasible => write!(f, "Infeasible"),
        }
    }
}
impl std::error::Error for LsapError {}

#[derive(Debug)]
pub enum MasterError {
    ImageError(image::ImageError),
    GridSizeMismatch {
        grid_size: (u32, u32),
        master_dimensions: (u32, u32),
        cell_size: (u32, u32),
    },
}
impl fmt::Display for MasterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MasterError::ImageError(err) => write!(f, "Image error: {}", err),
            MasterError::GridSizeMismatch {
                grid_size,
                master_dimensions,
                cell_size,
            } => write!(
                f,
                "Invalid grid shape {}x{}, the master dimensions: {}x{} are not divisible by the cell size: {}x{}", 
                grid_size.0, grid_size.1, master_dimensions.0, master_dimensions.1, cell_size.0, cell_size.1
            ),
        }
    }
}
impl From<image::ImageError> for MasterError {
    fn from(err: image::ImageError) -> Self {
        MasterError::ImageError(err)
    }
}
impl std::error::Error for MasterError {}

#[derive(Debug)]
pub enum MosaicError {
    /// The size of the distance matrix [crate::DistanceMatrix] does not match the number of
    /// [master cells](crate::master::Master::cells) and the nubmer of [tiles](crate::Mosaic::tiles).
    DistanceMatrixSizeMismatch {
        expected: (usize, usize),
        found: (usize, usize),
    },
    /// The tile images were not the same size as the grid cells.
    TileSizeMismatch {
        expected: (u32, u32),
        found: (u32, u32),
    },
    /// Not enough tiles were provided for the grid size and max tile occurrences.
    InsufficientTiles {
        provided: usize,
        required: usize,
        max_occurrences: usize,
    },
    /// The tile index is invalid.
    InvalidTileIndex(usize),
    /// A custom error message for specific issues.
    Custom(String),

    /// An error occurred while reading the master or tile images.
    ImageError(image::ImageError),
    /// A general I/O error occurred.
    IoError(io::Error),

    MasterError(MasterError),
    LsapError(LsapError),
}

impl fmt::Display for MosaicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MosaicError::TileSizeMismatch { expected, found } => write!(
                f,
                "Tile size mismatch: expected {}x{}, but found {}x{}",
                expected.0, expected.1, found.0, found.1
            ),
            MosaicError::InsufficientTiles {
                provided,
                required,
                max_occurrences,
            } => write!(
                f,
                "Not enough tiles provided: {} tiles for {} required with max occurrences of {}",
                provided, required, max_occurrences
            ),
            MosaicError::DistanceMatrixSizeMismatch { expected, found } => write!(
                f,
                "Distance matrix size mismatch: expected {}x{}, but found {}x{}",
                expected.0, expected.1, found.0, found.1
            ),
            MosaicError::Custom(msg) => write!(f, "{}", msg),
            MosaicError::InvalidTileIndex(index) => write!(f, "Invalid tile index: {}", index),

            MosaicError::MasterError(err) => write!(f, "Master error: {}", err),
            MosaicError::LsapError(err) => write!(f, "Lsap error: {}", err),

            MosaicError::ImageError(err) => write!(f, "Image error: {}", err),
            MosaicError::IoError(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for MosaicError {}

impl From<image::ImageError> for MosaicError {
    fn from(err: image::ImageError) -> Self {
        MosaicError::ImageError(err)
    }
}

impl From<io::Error> for MosaicError {
    fn from(err: io::Error) -> Self {
        MosaicError::IoError(err)
    }
}

impl From<&str> for MosaicError {
    fn from(msg: &str) -> Self {
        MosaicError::Custom(msg.to_string())
    }
}

impl From<String> for MosaicError {
    fn from(msg: String) -> Self {
        MosaicError::Custom(msg)
    }
}

impl From<MasterError> for MosaicError {
    fn from(err: MasterError) -> Self {
        MosaicError::MasterError(err)
    }
}

impl From<LsapError> for MosaicError {
    fn from(err: LsapError) -> Self {
        MosaicError::Custom(format!("Lsap error: {}", err))
    }
}
