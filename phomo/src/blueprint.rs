use image::{GenericImage, RgbImage};
use log::info;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Blueprint {
    pub cells: Vec<Cell>,
    pub cell_width: u32,
    pub cell_height: u32,
    pub grid_width: u32,
    pub grid_height: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Cell {
    pub tile_index: usize,
    pub x: u32,
    pub y: u32,
}

/// A serializable struct which represents a [crate::mosaic::Mosaic] that has yet to be rendered.
impl Blueprint {
    /// Render the [Blueprint].
    pub fn render(&self, master_img: &RgbImage, tiles: &[RgbImage]) -> Result<RgbImage, Error> {
        let mut mosaic_img = RgbImage::new(master_img.width(), master_img.height());
        info!(
            "Building mosaic, size: {}x{}, cell size: {}x{}, grid size: {}x{}",
            mosaic_img.width(),
            mosaic_img.height(),
            self.cell_width,
            self.cell_height,
            self.grid_width,
            self.grid_height
        );

        for cell in self.cells.iter() {
            let tile = tiles.get(cell.tile_index).ok_or_else(|| {
                format!("Tile index {} out of range", cell.tile_index).to_string()
            })?;
            mosaic_img.copy_from(tile, cell.x, cell.y)?;
        }
        Ok(mosaic_img)
    }
}
