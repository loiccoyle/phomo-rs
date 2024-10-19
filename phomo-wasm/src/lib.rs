use base64::{engine::general_purpose, Engine as _};
use image::RgbImage;
use phomo::{metrics, utils, Blueprint, ColorMatch, Mosaic as MosaicRs};
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

// Wrapper for Mosaic struct
#[wasm_bindgen]
pub struct Mosaic {
    master_img: RgbImage,
    tile_imgs: Vec<RgbImage>,
    mosaic: MosaicRs,
}

#[wasm_bindgen]
impl Mosaic {
    #[wasm_bindgen(constructor)]
    pub fn new(
        master_img_data: &[u8],
        tile_imgs_data: js_sys::Array,
        grid_width: u32,
        grid_height: u32,
        tile_resize: Option<ResizeType>,
    ) -> Result<Mosaic, JsValue> {
        // Load master image
        let master_img = image::load_from_memory(master_img_data)
            .map_err(|err| JsValue::from(err.to_string()))?
            .to_rgb8();

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
                        utils::crop_imm_centered(&img, cell_width, cell_height).to_image()
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
            master_img,
            tile_imgs,
            mosaic,
        })
    }

    // Method to equalize the master and tile images
    pub fn equalize(&mut self) {
        self.master_img = self.master_img.equalize();
        self.tile_imgs = self.tile_imgs.equalize();
    }

    // Method to transfer master image palette to tile images
    #[wasm_bindgen(js_name = transferMasterToTiles)]
    pub fn transfer_master_to_tiles(&mut self) {
        self.tile_imgs = self.tile_imgs.match_palette(&self.master_img);
    }

    // Method to transfer tile images palette to master image
    #[wasm_bindgen(js_name = transferTilesToMaster)]
    pub fn transfer_tiles_to_master(&mut self) {
        self.master_img = self.master_img.match_palette(&self.tile_imgs);
    }

    fn distance_matrix_with_metric(&self, metric_type: &str) -> Result<Vec<i64>, JsValue> {
        let metric = match metric_type {
            "NormL1" => metrics::norm_l1,
            "NormL2" => metrics::norm_l2,
            _ => return Err(JsValue::from("Unsupported metric type")),
        };

        Ok(self.mosaic.distance_matrix_with_metric(metric))
    }

    // Build the mosaic after applying palette matching or equalization
    pub fn build(&self, metric_type: &str) -> Result<String, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;

        let mosaic_img = self
            .mosaic
            .build(d_matrix)
            .map_err(|err| JsValue::from(err.to_string()))?;

        to_base64(mosaic_img)
    }

    #[wasm_bindgen(js_name = buildBlueprint)]
    pub fn build_blueprint(&self, metric_type: &str) -> Result<JsValue, JsValue> {
        let d_matrix = self.distance_matrix_with_metric(metric_type)?;
        let mosaic = self
            .mosaic
            .build_blueprint(d_matrix)
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(serde_wasm_bindgen::to_value(&mosaic).unwrap())
    }

    #[wasm_bindgen(js_name = renderBlueprint)]
    pub fn render_blueprint(&self, blueprint: JsValue) -> Result<String, JsValue> {
        let blueprint = serde_wasm_bindgen::from_value::<Blueprint>(blueprint)?;
        let mosaic_img = blueprint
            .render(&self.master_img, &self.tile_imgs)
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
