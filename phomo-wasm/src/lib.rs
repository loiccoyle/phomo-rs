use base64::{engine::general_purpose, Engine as _};
use image::RgbImage;
use phomo::{
    metrics, utils, Auction, Blueprint, ColorMatch, Greedy, Master as MasterRs, Mosaic as MosaicRs,
};
use phomo::{DistanceMatrix, SolverConfig};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
extern crate wasm_logger;

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub enum ResizeType {
    Crop,
    Resize,
}

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

    // Method to equalize the master and tile images
    pub fn equalize(&mut self) {
        self.inner.master.img = self.inner.master.img.equalize();
        self.inner.tiles = self.inner.tiles.equalize();
    }

    // Method to transfer master image palette to tile images
    #[wasm_bindgen(js_name = transferMasterToTiles)]
    pub fn transfer_master_to_tiles(&mut self) {
        self.inner.tiles = self.inner.tiles.match_palette(&self.inner.master.img);
    }

    // Method to transfer tile images palette to master image
    #[wasm_bindgen(js_name = transferTilesToMaster)]
    pub fn transfer_tiles_to_master(&mut self) -> Result<(), JsValue> {
        self.inner.master = MasterRs::from_image(
            self.inner.master.img.match_palette(&self.inner.tiles),
            self.inner.grid_size,
        )
        .map_err(|err| JsValue::from(err.to_string()))?;
        Ok(())
    }

    fn distance_matrix_with_metric(&self, metric_type: &str) -> Result<DistanceMatrix, JsValue> {
        let metric = match metric_type {
            "NormL1" => metrics::norm_l1,
            "NormL2" => metrics::norm_l2,
            _ => return Err(JsValue::from("Unsupported metric type")),
        };

        Ok(self.inner.distance_matrix_with_metric(metric))
    }

    // Build the mosaic after applying palette matching or equalization
    pub fn build(&self, metric_type: &str) -> Result<String, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;

        let mosaic_img = self
            .inner
            .build(d_matrix, self.solver_config.clone())
            .map_err(|err| JsValue::from(err.to_string()))?;

        to_base64(mosaic_img)
    }

    #[wasm_bindgen(js_name = buildGreedy)]
    pub fn build_greedy(&self, metric_type: &str) -> Result<String, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;

        let mosaic_img = self
            .inner
            .build_with_solver(d_matrix, Greedy::new(self.solver_config.clone()))
            .map_err(|err| JsValue::from(err.to_string()))?;

        to_base64(mosaic_img)
    }

    #[wasm_bindgen(js_name = buildAuction)]
    pub fn build_auction(&self, metric_type: &str) -> Result<String, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;

        let mosaic_img = self
            .inner
            .build_with_solver(d_matrix, Auction::new(1, self.solver_config.clone()))
            .map_err(|err| JsValue::from(err.to_string()))?;

        to_base64(mosaic_img)
    }

    #[wasm_bindgen(js_name = getTiles)]
    pub fn get_tiles(&self) -> Result<Vec<String>, JsValue> {
        let mut tiles = Vec::with_capacity(self.inner.tiles.len());
        for tile in self.inner.tiles.iter() {
            tiles.push(to_base64(tile.clone())?);
        }

        Ok(tiles)
    }

    #[wasm_bindgen(js_name = getMaster)]
    pub fn get_master(&self) -> Result<String, JsValue> {
        to_base64(self.inner.master.img.clone())
    }

    #[wasm_bindgen(js_name = buildBlueprint)]
    pub fn build_blueprint(&self, metric_type: &str) -> Result<JsValue, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;

        let mosaic = self
            .inner
            .build_blueprint(d_matrix, self.solver_config.clone())
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&mosaic)?)
    }

    #[wasm_bindgen(js_name = buildBlueprintGreedy)]
    pub fn build_blueprint_greedy(&self, metric_type: &str) -> Result<JsValue, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;

        let mosaic = self
            .inner
            .build_blueprint_with_solver(d_matrix, Greedy::new(self.solver_config.clone()))
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&mosaic)?)
    }
    #[wasm_bindgen(js_name = buildBlueprintAuction)]
    pub fn build_blueprint_auction(&self, metric_type: &str) -> Result<JsValue, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;

        let mosaic = self
            .inner
            .build_blueprint_with_solver(d_matrix, Auction::new(1, self.solver_config.clone()))
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&mosaic)?)
    }

    #[wasm_bindgen(js_name = renderBlueprint)]
    pub fn render_blueprint(&self, blueprint: JsValue) -> Result<String, JsValue> {
        let blueprint = serde_wasm_bindgen::from_value::<Blueprint>(blueprint)?;
        let mosaic_img = blueprint
            .render(&self.inner.master.img, &self.inner.tiles)
            .map_err(|err| JsValue::from(err.to_string()))?;

        to_base64(mosaic_img)
    }
}

fn to_base64(img: RgbImage) -> Result<String, JsValue> {
    // Convert the mosaic image to base64 encoded PNG
    let mut buf: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|err| JsValue::from(err.to_string()))?;

    Ok(general_purpose::STANDARD.encode(buf))
}
