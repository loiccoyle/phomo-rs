use std::env;
use std::path::Path;
use std::path::PathBuf;

extern crate image;

use phomo::read_images_from_dir_resized;
use phomo::ColorMatch;
use phomo::Mosaic;

fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("mosaic")
}

fn tile_dir() -> PathBuf {
    // 1000 20x20
    test_dir().join("faces")
}

fn master_file() -> PathBuf {
    // 256x256
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("master")
        .join("master.png")
}

fn open_expected<P: AsRef<Path>>(img: &image::RgbImage, expected: P) -> image::RgbImage {
    if env::var("PHOMO_UPDATE_EXPECTED").is_ok() {
        img.save(expected.as_ref()).unwrap();
    }

    image::open(expected).unwrap().to_rgb8()
}

#[test]
fn build_mosaic() {
    let tile_dir = tile_dir();
    let master_file = master_file();

    let tile_imgs =
        read_images_from_dir_resized(tile_dir, 32, 32, image::imageops::FilterType::Nearest)
            .unwrap();
    let master_img = image::open(master_file).unwrap().to_rgb8();

    let result = Mosaic::from_images(master_img, tile_imgs, (8, 8));
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(&mosaic_img, test_dir().join("mosaic_12_12.png"));
    assert_eq!(mosaic_img, expected);
}

#[test]
fn build_mosaic_equalized() {
    let tile_dir = tile_dir();
    let master_file = master_file();

    let mut tile_imgs =
        read_images_from_dir_resized(tile_dir, 32, 32, image::imageops::FilterType::Nearest)
            .unwrap();
    let mut master_img = image::open(master_file).unwrap().to_rgb8();

    tile_imgs = tile_imgs.equalize();
    master_img = master_img.equalize();

    let result = Mosaic::from_images(master_img, tile_imgs, (8, 8));
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(&mosaic_img, test_dir().join("mosaic_12_12_equalized.png"));
    assert_eq!(mosaic_img, expected);
}

#[test]
fn build_mosaic_match_tiles_to_master() {
    let tile_dir = tile_dir();
    let master_file = master_file();

    let mut tile_imgs =
        read_images_from_dir_resized(tile_dir, 32, 32, image::imageops::FilterType::Nearest)
            .unwrap();
    let master_img = image::open(master_file).unwrap().to_rgb8();

    tile_imgs = tile_imgs.match_palette(&master_img);

    let result = Mosaic::from_images(master_img, tile_imgs, (8, 8));
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(
        &mosaic_img,
        test_dir().join("mosaic_12_12_match_tiles_to_master.png"),
    );
    assert_eq!(mosaic_img, expected);
}

#[test]
fn build_mosaic_match_master_to_tiles() {
    let tile_dir = tile_dir();
    let master_file = master_file();

    let tile_imgs =
        read_images_from_dir_resized(tile_dir, 32, 32, image::imageops::FilterType::Nearest)
            .unwrap();
    let mut master_img = image::open(master_file).unwrap().to_rgb8();

    master_img = master_img.match_palette(&tile_imgs);
    let expected = open_expected(&master_img, test_dir().join("master_matched_to_faces.png"));
    assert_eq!(master_img, expected);

    let result = Mosaic::from_images(master_img, tile_imgs, (8, 8));
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(
        &mosaic_img,
        test_dir().join("mosaic_12_12_match_master_to_tiles.png"),
    );
    assert_eq!(mosaic_img, expected);
}
