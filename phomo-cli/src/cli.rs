extern crate clap;
use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use clap_verbosity_flag::Verbosity;

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
#[clap(author = "Loic Coyle")]
#[command(group(
    clap::ArgGroup::new("transfer")
        .required(false)  // Set to `true` if you want one flag to be mandatory
        .args(&["transfer_tiles_to_master", "transfer_master_to_tiles"]),
))]
#[command(group(
    clap::ArgGroup::new("tile_resize")
        .required(false)  // Set to `true` if you want one flag to be mandatory
        .args(&["crop_tiles", "resize_tiles"]),
))]
pub struct Arguments {
    /// Master image.
    #[arg( value_hint=clap::ValueHint::FilePath)]
    pub master_file: PathBuf,
    /// Tile directory.
    #[arg(value_hint=clap::ValueHint::DirPath)]
    pub tile_dir: PathBuf,
    /// Output mosaic file.
    #[arg(value_hint=clap::ValueHint::FilePath)]
    pub output: PathBuf,

    /// Grid size, the number of tiles along the width and height.
    ///
    /// If not provided, the grid size will be set to a sane value depending on the number of tiles images.
    #[arg(short = 'g', long, value_name = "WIDTH,HEIGHT")]
    pub grid_size: Option<TwoNumbers>,
    /// Crop tiles to grid cell size.
    #[arg(long)]
    pub crop_tiles: bool,
    /// Resize tiles to grid cell size.
    #[arg(long)]
    pub resize_tiles: bool,

    /// Equalize the master and tile image color distributions.
    #[arg(long)]
    pub equalize: bool,
    /// Transfer the color palette of the master image to the tile images.
    #[arg(long)]
    pub transfer_master_to_tiles: bool,
    /// Transfer the color palette of the tile images to the master image.
    #[arg(long)]
    pub transfer_tiles_to_master: bool,

    #[command(flatten)]
    pub verbose: Verbosity,
}
