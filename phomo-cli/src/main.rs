use std::error::Error;

extern crate env_logger;
use clap::Parser;
use log::info;
use phomo::{
    read_images_from_dir, read_images_from_dir_cropped, read_images_from_dir_resized, ColorMatch,
    Mosaic,
};

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the CLI arguments
    let args = cli::Arguments::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let mut master_img = image::open(args.master_file)
        .map_err(|e| format!("Failed to read master image: {}", e))?
        .to_rgb8();
    info!(
        "Master image size: {}x{}",
        master_img.width(),
        master_img.height()
    );

    let tile_count = args
        .tile_dir
        .read_dir()?
        .filter_map(|entry| {
            let entry = entry.ok()?; // Skip invalid entries
            let path = entry.path();
            if path.is_file() {
                // Try to get the image dimensions, returns None if it's not an image
                image::image_dimensions(&path).ok()
            } else {
                None
            }
        })
        .count();
    info!("Tile count: {}", tile_count);

    // determine a resonable grid size
    let (grid_width, grid_height) = match args.grid_size {
        Some(cli::TwoNumbers(width, height)) => (width, height),
        None => {
            // we leave a few tiles to be unused so get better assignments
            let grid_dim = (tile_count as f32 * 0.8).sqrt().round() as u32;
            (grid_dim, grid_dim)
        }
    };
    info!("Grid size: {}x{}", grid_width, grid_height);

    let (cell_width, cell_height) = (
        master_img.width() / grid_width,
        master_img.height() / grid_height,
    );
    info!("Cell size: {}x{}", cell_width, cell_height);

    // Read tile images from the directory
    let mut tile_imgs = if args.crop_tiles {
        read_images_from_dir_cropped(&args.tile_dir, cell_width, cell_height)
    } else if args.resize_tiles {
        read_images_from_dir_resized(
            &args.tile_dir,
            cell_width,
            cell_height,
            image::imageops::FilterType::Nearest,
        )
    } else {
        read_images_from_dir(&args.tile_dir)
    }
    .map_err(|e| format!("Failed to crop tile images: {}", e))?;

    // Handle the different transformations
    if args.equalize {
        master_img = master_img.equalize();
        tile_imgs = tile_imgs.equalize();
    }

    if args.transfer_master_to_tiles {
        tile_imgs = tile_imgs.match_palette(&master_img);
    } else if args.transfer_tiles_to_master {
        master_img = master_img.match_palette(&tile_imgs);
    }

    if !tile_imgs
        .iter()
        .all(|tile| tile.dimensions() == (cell_width, cell_height))
    {
        return Err(format!(
            "All tile images must have dimensions equal to the grid cell size: {}x{}, consider using the --crop-tiles or --resize-tiles flags",
            cell_width, cell_height
        )
        .into());
    };

    // Create the mosaic
    let mosaic = Mosaic::from_images(master_img, tile_imgs, (grid_width, grid_height))
        .map_err(|e| format!("Failed to create mosaic: {}", e))?;

    let metric = match args.metric {
        cli::Metric::NormL1 => phomo::metrics::norm_l1,
        cli::Metric::NormL2 => phomo::metrics::norm_l2,
    };
    // Compute the distance matrix
    let mut d_matrix = mosaic.distance_matrix_with_metric(metric);
    if args.n_appearances > 1 {
        d_matrix = d_matrix.with_repeat_tiles(args.n_appearances as usize);
    }

    // Build the mosaic image
    let mosaic_img = mosaic
        .build(d_matrix)
        .map_err(|e| format!("Failed to build mosaic image: {}", e))?;

    // Save the final mosaic image to output
    mosaic_img
        .save(&args.output)
        .map_err(|e| format!("Failed to save mosaic image: {}", e))?;

    info!("Mosaic image created successfully: {:?}", args.output);
    Ok(())
}
