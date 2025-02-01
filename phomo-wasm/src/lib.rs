use base64::{engine::general_purpose, Engine as _};
use image::RgbImage;
use phomo::{
    metrics, utils, Auction, Blueprint, ColorMatch, Greedy, Hungarian, Master as MasterRs,
    Mosaic as MosaicRs,
};
use phomo::{DistanceMatrix, SolverConfig};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
extern crate wasm_logger;

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
}

/// Metric types
#[wasm_bindgen]
pub enum MetricType {
    NormL1,
    NormL2,
    LuminanceL1,
    LuminanceL2,
    AvgColor,
}

/// Tile resizing types
#[wasm_bindgen]
pub enum ResizeType {
    Crop,
    Resize,
}

/// Tile to cell solvers.
#[wasm_bindgen]
pub enum Solver {
    Greedy,
    Hungarian,
    Auction,
}

/// Overlay a grid on the master image
#[wasm_bindgen(js_name = overlayGrid)]
pub fn overlay_grid(
    master_img_data: &[u8],
    grid_width: u32,
    grid_height: u32,
) -> Result<String, JsValue> {
    let master_img = image::load_from_memory(master_img_data)
        .map_err(|err| JsValue::from(err.to_string()))?
        .to_rgb8();
    let master = MasterRs::from_image(master_img, (grid_width, grid_height))
        .map_err(|err| JsValue::from(err.to_string()))?;
    let grid = master
        .overlay_grid()
        .map_err(|err| JsValue::from(err.to_string()))?;

    to_base64(grid)
}

// Wrapper for Mosaic struct
#[wasm_bindgen]
pub struct Mosaic {
    inner: MosaicRs,
    solver_config: SolverConfig,
}

#[wasm_bindgen]
impl Mosaic {
    /// Create a new Mosaic instance.
    ///
    /// # Arguments
    /// - `master_img_data`: The master image data as a byte array.
    /// - `tile_imgs_data`: The tile images data as a JS array of byte arrays.
    /// - `grid_width`: The number of grid columns.
    /// - `grid_height`: The number of grid rows.
    /// - `max_tile_occurrences`: The maximum number of times a tile can be used in the mosaic.
    /// - `tile_resize`: The type of tile resizing to apply.
    /// - ''master_resize`: The desired size of the master image after resizing.
    #[wasm_bindgen(constructor)]
    pub fn new(
        master_img_data: &[u8],
        tile_imgs_data: js_sys::Array,
        grid_width: u32,
        grid_height: u32,
        max_tile_occurrences: usize,
        tile_resize: Option<ResizeType>,
        master_resize: Option<Vec<u32>>,
    ) -> Result<Mosaic, JsValue> {
        // Load master image
        let mut master_img = image::load_from_memory(master_img_data)
            .map_err(|err| JsValue::from(err.to_string()))?
            .to_rgb8();

        if let Some(master_resize) = master_resize {
            master_img = image::imageops::resize(
                &master_img,
                master_resize[0],
                master_resize[1],
                image::imageops::FilterType::Nearest,
            );
        }

        // Load tile images
        let tile_imgs = (0..tile_imgs_data.length())
            .map(|i| {
                let data = js_sys::Uint8Array::new(&tile_imgs_data.get(i)).to_vec();
                let img = image::load_from_memory(&data)
                    .map_err(|err| JsValue::from(err.to_string()))
                    .unwrap()
                    .to_rgb8();

                let cell_width = master_img.width() / grid_width;
                let cell_height = master_img.height() / grid_height;
                match tile_resize {
                    Some(ResizeType::Resize) => image::imageops::resize(
                        &img,
                        cell_width,
                        cell_height,
                        image::imageops::FilterType::Nearest,
                    ),
                    Some(ResizeType::Crop) => {
                        if cell_width > img.width() || cell_height > img.height() {
                            panic!("Could not crop tile to cell size as the cell size is larger than the tile size.");
                        }
                            utils::crop_cover(&img, cell_width, cell_height, image::imageops::FilterType::Nearest)
                        }
                    None => img,
                }
            })
            .collect::<Vec<_>>();

        let mosaic = MosaicRs::from_images(
            master_img.clone(),
            tile_imgs.clone(),
            (grid_width, grid_height),
        )
        .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(Mosaic {
            inner: mosaic,
            solver_config: SolverConfig {
                max_tile_occurrences,
            },
        })
    }

    /// Equalize the master and tile images
    pub fn equalize(&mut self) {
        self.inner.master.img = self.inner.master.img.equalize();
        self.inner.tiles = self.inner.tiles.equalize();
    }

    /// Transfer master image palette to tile images
    #[wasm_bindgen(js_name = transferMasterToTiles)]
    pub fn transfer_master_to_tiles(&mut self) {
        self.inner.tiles = self.inner.tiles.match_palette(&self.inner.master.img);
    }

    /// Transfer tile images palette to master image
    #[wasm_bindgen(js_name = transferTilesToMaster)]
    pub fn transfer_tiles_to_master(&mut self) -> Result<(), JsValue> {
        self.inner.master = MasterRs::from_image(
            self.inner.master.img.match_palette(&self.inner.tiles),
            self.inner.grid_size,
        )
        .map_err(|err| JsValue::from(err.to_string()))?;
        Ok(())
    }

    fn distance_matrix_with_metric(
        &self,
        metric_type: MetricType,
    ) -> Result<DistanceMatrix, JsValue> {
        let metric = match metric_type {
            MetricType::NormL1 => metrics::norm_l1,
            MetricType::NormL2 => metrics::norm_l2,
            MetricType::LuminanceL1 => metrics::luminance_l1,
            MetricType::LuminanceL2 => metrics::luminance_l2,
            MetricType::AvgColor => metrics::avg_color,
        };

        Ok(self.inner.distance_matrix_with_metric(metric))
    }

    /// Build the mosaic img using the Hungarian algorithm and return it as base64 encoded PNG.
    pub fn build(&self, metric_type: MetricType) -> Result<String, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;

        let mosaic_img = self
            .inner
            .build(d_matrix, self.solver_config.clone())
            .map_err(|err| JsValue::from(err.to_string()))?;

        to_base64(mosaic_img)
    }

    /// Build the mosaic img using the specified solver and return it as base64 encoded PNG.
    #[wasm_bindgen(js_name = buildWithSolver)]
    pub fn build_with_solver(
        &self,
        metric_type: MetricType,
        solver: Solver,
    ) -> Result<String, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;
        let mosaic_img = match solver {
            Solver::Greedy => self
                .inner
                .build_with_solver(d_matrix, Greedy::new(self.solver_config.clone())),
            Solver::Hungarian => self
                .inner
                .build_with_solver(d_matrix, Hungarian::new(self.solver_config.clone())),
            Solver::Auction => self
                .inner
                .build_with_solver(d_matrix, Auction::new(1, self.solver_config.clone())),
        }
        .map_err(|err| JsValue::from(err.to_string()))?;

        to_base64(mosaic_img)
    }

    /// Get the tile images as base64 encoded PNGs.
    #[wasm_bindgen(js_name = getTiles)]
    pub fn get_tiles(&self) -> Result<Vec<String>, JsValue> {
        let mut tiles = Vec::with_capacity(self.inner.tiles.len());
        for tile in self.inner.tiles.iter() {
            tiles.push(to_base64(tile.clone())?);
        }

        Ok(tiles)
    }

    /// Get the master image as base64 encoded PNG.
    #[wasm_bindgen(js_name = getMaster)]
    pub fn get_master(&self) -> Result<String, JsValue> {
        to_base64(self.inner.master.img.clone())
    }

    /// Build the mosaic blueprint using the Hungarian algorithm and return it as JSON.
    #[wasm_bindgen(js_name = buildBlueprint)]
    pub fn build_blueprint(&self, metric_type: MetricType) -> Result<JsValue, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;

        let mosaic = self
            .inner
            .build_blueprint(d_matrix, self.solver_config.clone())
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&mosaic)?)
    }

    /// Build the mosaic blueprint using the specified solver and return it as JSON.
    #[wasm_bindgen(js_name = buildBlueprintWithSolver)]
    pub fn build_blueprint_with_solver(
        &self,
        metric_type: MetricType,
        solver: Solver,
    ) -> Result<JsValue, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;
        let blueprint = match solver {
            Solver::Greedy => self
                .inner
                .build_blueprint_with_solver(d_matrix, Greedy::new(self.solver_config.clone())),
            Solver::Hungarian => self
                .inner
                .build_blueprint_with_solver(d_matrix, Hungarian::new(self.solver_config.clone())),
            Solver::Auction => self
                .inner
                .build_blueprint_with_solver(d_matrix, Auction::new(1, self.solver_config.clone())),
        }
        .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&blueprint)?)
    }

    /// Render the mosaic blueprint and return it as base64 encoded PNG.
    #[wasm_bindgen(js_name = renderBlueprint)]
    pub fn render_blueprint(&self, blueprint: JsValue) -> Result<String, JsValue> {
        let blueprint = serde_wasm_bindgen::from_value::<Blueprint>(blueprint)?;
        let mosaic_img = blueprint
            .render(&self.inner.master.img, &self.inner.tiles)
            .map_err(|err| JsValue::from(err.to_string()))?;

        to_base64(mosaic_img)
    }
}

/// Convert the mosaic image to base64 encoded PNG.
fn to_base64(img: RgbImage) -> Result<String, JsValue> {
    let mut buf: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|err| JsValue::from(err.to_string()))?;

    Ok(general_purpose::STANDARD.encode(buf))
}
