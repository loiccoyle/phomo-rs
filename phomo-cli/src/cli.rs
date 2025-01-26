use std::{fmt::Display, path::PathBuf, str::FromStr};

extern crate clap;
use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(clap::ValueEnum, Clone, Debug)]
pub(crate) enum Metric {
    NormL1,
    NormL2,
    AvgColor,
    LuminanceL1,
    LuminanceL2,
}

impl Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Metric::NormL1 => write!(f, "norm-l1"),
            Metric::NormL2 => write!(f, "norm-l2"),
            Metric::AvgColor => write!(f, "avg-color"),
            Metric::LuminanceL1 => write!(f, "luminance-l1"),
            Metric::LuminanceL2 => write!(f, "luminance-l2"),
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub(crate) enum Solver {
    Greedy,
    Auction,
    Hungarian,
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solver::Greedy => write!(f, "greedy"),
            Solver::Auction => write!(f, "auction"),
            Solver::Hungarian => write!(f, "hungarian"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TwoNumbers(pub u32, pub u32);

impl FromStr for TwoNumbers {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Expected two numbers separated by a comma".into());
        }

        let first = parts[0]
            .trim()
            .parse::<u32>()
            .map_err(|_| "Invalid number for first value")?;
        let second = parts[1]
            .trim()
            .parse::<u32>()
            .map_err(|_| "Invalid number for second value")?;
        Ok(TwoNumbers(first, second))
    }
}

#[derive(Parser, Debug)]
#[command(version, author = "Loic Coyle")]
#[command(group(
    clap::ArgGroup::new("transfer")
        .required(false)
        .args(&["transfer_tiles_to_master", "transfer_master_to_tiles"]),
))]
#[command(group(
    clap::ArgGroup::new("tile_resize")
        .required(false)
        .args(&["crop_tiles", "resize_tiles"]),
))]
pub(crate) struct Arguments {
    /// Master image.
    #[arg(value_hint=clap::ValueHint::FilePath)]
    pub(crate) master_file: PathBuf,
    /// Tile directory.
    #[arg(value_hint=clap::ValueHint::DirPath)]
    pub(crate) tile_dir: PathBuf,
    /// Output mosaic file.
    #[arg(value_hint=clap::ValueHint::FilePath)]
    pub(crate) output: PathBuf,

    /// Grid size, the number of tiles along the width and height.
    ///
    /// If not provided, the grid size will be set to a sane value depending on the number of tiles images.
    #[arg(short = 'g', long, value_name = "WIDTH,HEIGHT")]
    pub(crate) grid_size: Option<TwoNumbers>,
    /// The number of times a tile can appear in the mosaic.
    #[arg(short = 'n', long, default_value_t = 1)]
    pub(crate) n_appearances: usize,
    /// Crop tiles to grid cell size.
    #[arg(long)]
    pub(crate) crop_tiles: bool,
    /// Resize tiles to grid cell size.
    #[arg(long)]
    pub(crate) resize_tiles: bool,

    /// Equalize the master and tile image color distributions.
    #[arg(long)]
    pub(crate) equalize: bool,
    /// Transfer the color palette of the master image to the tile images.
    #[arg(long)]
    pub(crate) transfer_master_to_tiles: bool,
    /// Transfer the color palette of the tile images to the master image.
    #[arg(long)]
    pub(crate) transfer_tiles_to_master: bool,

    /// The solver to use to compute the tile to cell assignments.
    #[arg(long, default_value_t = Solver::Hungarian)]
    pub(crate) solver: Solver,
    /// The distance metric to use.
    #[arg(long, default_value_t = Metric::NormL1)]
    pub(crate) metric: Metric,

    #[command(flatten)]
    pub(crate) verbose: Verbosity,
}
