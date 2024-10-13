use base64::{engine::general_purpose, Engine as _};
use image::RgbImage;
use phomo::{metrics, utils, ColorMatch, Mosaic as MosaicRs};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
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
    mosaic: Option<MosaicRs>,
    grid_width: u32,
    grid_height: u32,
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

                match tile_resize {
                    Some(ResizeType::Resize) => image::imageops::resize(
                        &img,
                        master_img.width() / grid_width,
                        master_img.height() / grid_height,
                        image::imageops::FilterType::Nearest,
                    ),
                    Some(ResizeType::Crop) => utils::crop_imm_centered(
                        &img,
                        master_img.width() / grid_width,
                        master_img.height() / grid_height,
                    )
                    .to_image(),
                    None => img,
                }
            })
            .collect();

        Ok(Mosaic {
            master_img,
            tile_imgs,
            mosaic: None,
            grid_width,
            grid_height,
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

    // Build the mosaic after applying palette matching or equalization
    pub fn build(&mut self, metric_type: &str) -> Result<String, JsValue> {
        // Ensure the mosaic is constructed only once after transformations
        self.mosaic = Some(
            MosaicRs::from_images(
                self.master_img.clone(),
                self.tile_imgs.clone(),
                (self.grid_width, self.grid_height),
            )
            .map_err(|err| JsValue::from(err.to_string()))?,
        );

        let mosaic = self.mosaic.as_ref().unwrap();

        // Choose the metric based on the input string
        let metric = match metric_type {
            "NormL1" => metrics::norm_l1,
            "NormL2" => metrics::norm_l2,
            _ => return Err(JsValue::from("Unsupported metric type")),
        };

        let d_matrix = mosaic.distance_matrix_with_metric(metric);

        let mosaic_img = mosaic
            .build(d_matrix)
            .map_err(|err| JsValue::from(err.to_string()))?;

        // Convert the mosaic image to base64 encoded PNG
        let mut buf: Vec<u8> = Vec::new();
        mosaic_img
            .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(general_purpose::STANDARD.encode(buf))
    }
}

// Utility to load an image from base64
// #[wasm_bindgen]
// pub fn load_image_from_base64(base64_data: &str) -> Result<DynamicImage, JsValue> {
//     let decoded_data = base64::decode(base64_data).map_err(|err| JsValue::from(err.to_string()))?;
//     image::load_from_memory(&decoded_data).map_err(|err| JsValue::from(err.to_string()))
// }
