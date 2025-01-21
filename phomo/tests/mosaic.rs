use std::env;
use std::path::Path;
use std::path::PathBuf;

extern crate image;
use image::RgbImage;

use phomo::read_images_from_dir_resized;
#[cfg(feature = "blueprint")]
use phomo::Blueprint;
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

fn setup_imgs() -> (Vec<RgbImage>, RgbImage) {
    let tile_dir = tile_dir();
    let master_file = master_file();

    let tile_imgs =
        read_images_from_dir_resized(tile_dir, 16, 16, image::imageops::FilterType::Nearest)
            .unwrap();
    let master_img = image::open(master_file).unwrap().to_rgb8();
    (tile_imgs, master_img)
}

#[test]
fn build_mosaic() {
    let (tile_imgs, master_img) = setup_imgs();

    let result = Mosaic::from_images(master_img, tile_imgs, (16, 16), 1);
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.data.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(&mosaic_img, test_dir().join("mosaic_16_16.png"));
    assert!(kinda_same_imgs(mosaic_img, expected, 2.));
}

#[test]
fn build_mosaic_greedy() {
    let (tile_imgs, master_img) = setup_imgs();

    let result = Mosaic::from_images(master_img, tile_imgs, (16, 16), 1);
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.data.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build_greedy(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(&mosaic_img, test_dir().join("mosaic_16_16_greedy.png"));
    assert!(kinda_same_imgs(mosaic_img, expected, 2.));
}

#[test]
fn build_mosaic_repeats() {
    let (tile_imgs, master_img) = setup_imgs();

    let result = Mosaic::from_images(master_img, tile_imgs, (16, 16), 2);
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.data.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(&mosaic_img, test_dir().join("mosaic_16_16_repeats.png"));
    assert!(kinda_same_imgs(mosaic_img, expected, 2.));
}

#[test]
fn build_mosaic_greedy_repeats() {
    let (tile_imgs, master_img) = setup_imgs();

    let result = Mosaic::from_images(master_img, tile_imgs, (16, 16), 2);
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.data.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build_greedy(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(
        &mosaic_img,
        test_dir().join("mosaic_16_16_greedy_repeats.png"),
    );
    assert!(kinda_same_imgs(mosaic_img, expected, 2.));
}

#[test]
fn build_mosaic_equalized() {
    let (mut tile_imgs, mut master_img) = setup_imgs();

    tile_imgs = tile_imgs.equalize();
    master_img = master_img.equalize();

    let result = Mosaic::from_images(master_img, tile_imgs, (16, 16), 1);
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.data.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(&mosaic_img, test_dir().join("mosaic_16_16_equalized.png"));
    assert!(kinda_same_imgs(mosaic_img, expected, 2.));
}

#[test]
fn build_mosaic_match_tiles_to_master() {
    let (mut tile_imgs, master_img) = setup_imgs();

    tile_imgs = tile_imgs.match_palette(&master_img);

    let result = Mosaic::from_images(master_img, tile_imgs, (16, 16), 1);
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.data.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(
        &mosaic_img,
        test_dir().join("mosaic_16_16_match_tiles_to_master.png"),
    );
    assert!(kinda_same_imgs(mosaic_img, expected, 2.));
}

#[test]
fn build_mosaic_match_master_to_tiles() {
    let (tile_imgs, mut master_img) = setup_imgs();

    master_img = master_img.match_palette(&tile_imgs);
    let expected = open_expected(&master_img, test_dir().join("master_matched_to_faces.png"));
    assert!(kinda_same_imgs(master_img.clone(), expected, 2.));

    let result = Mosaic::from_images(master_img, tile_imgs, (16, 16), 1);
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.data.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let mosaic_img = mosaic.build(d_matrix).unwrap();
    assert_eq!(mosaic_img.dimensions(), mosaic.master.img.dimensions());
    let expected = open_expected(
        &mosaic_img,
        test_dir().join("mosaic_16_16_match_master_to_tiles.png"),
    );
    assert!(kinda_same_imgs(mosaic_img, expected, 2.));
}

// TODO: this test fails when running on GitHub Actions
#[cfg(feature = "blueprint")]
#[test]
fn build_mosaic_blueprint() {
    if std::env::var("CI").is_ok() {
        println!("Test skipped: Running on GitHub Actions.");
        return;
    }
    let (tile_imgs, master_img) = setup_imgs();

    let result = Mosaic::from_images(master_img, tile_imgs, (16, 16), 1);
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.data.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let result = mosaic.build_blueprint(d_matrix);
    assert!(result.is_ok());
    let blueprint = result.unwrap();
    let serialized = serde_json::to_string_pretty(&blueprint).unwrap();
    let expected_path = test_dir().join("mosaic_blueprint.json");
    if std::env::var("PHOMO_UPDATE_EXPECTED").is_ok() {
        std::fs::write(&expected_path, serialized).unwrap();
    }
    let expected_blueprint: Blueprint =
        serde_json::from_str(&std::fs::read_to_string(&expected_path).unwrap()).unwrap();
    // make sure the json is the same
    assert_eq!(expected_blueprint, blueprint);

    let result = blueprint.render(&mosaic.master.img, &mosaic.tiles);
    assert!(result.is_ok());
    let mosaic_img = result.unwrap();
    let expected = open_expected(
        &mosaic_img,
        test_dir().join("mosaic_blueprint_rendered.png"),
    );
    // make sure the rendered img is the same
    assert!(kinda_same_imgs(mosaic_img, expected, 2.));
}

// TODO: this test fails when running on GitHub Actions
#[cfg(feature = "blueprint")]
#[test]
fn build_mosaic_blueprint_greedy() {
    if std::env::var("CI").is_ok() {
        println!("Test skipped: Running on GitHub Actions.");
        return;
    }
    let (tile_imgs, master_img) = setup_imgs();

    let result = Mosaic::from_images(master_img, tile_imgs, (16, 16), 1);
    assert!(result.is_ok());
    let mosaic = result.unwrap();

    let d_matrix = mosaic.distance_matrix();
    assert_eq!(
        d_matrix.data.len(),
        mosaic.master.cells.len() * mosaic.tiles.len()
    );

    let result = mosaic.build_blueprint_greedy(d_matrix);
    assert!(result.is_ok());
    let blueprint = result.unwrap();
    let serialized = serde_json::to_string_pretty(&blueprint).unwrap();
    let expected_path = test_dir().join("mosaic_blueprint_greedy.json");
    if std::env::var("PHOMO_UPDATE_EXPECTED").is_ok() {
        std::fs::write(&expected_path, serialized).unwrap();
    }
    let expected_blueprint: Blueprint =
        serde_json::from_str(&std::fs::read_to_string(&expected_path).unwrap()).unwrap();
    // make sure the json is the same
    assert_eq!(expected_blueprint, blueprint);

    let result = blueprint.render(&mosaic.master.img, &mosaic.tiles);
    assert!(result.is_ok());
    let mosaic_img = result.unwrap();
    let expected = open_expected(
        &mosaic_img,
        test_dir().join("mosaic_blueprint_greedy_rendered.png"),
    );
    // make sure the rendered img is the same
    assert!(kinda_same_imgs(mosaic_img, expected, 2.));
}

/// compare two images return true if ther are close to being the same
fn kinda_same_imgs(img1: image::RgbImage, img2: image::RgbImage, tol: f64) -> bool {
    let diff = img1
        .as_raw()
        .iter()
        .zip(img2.as_raw())
        .map(|(a, b)| (*a).abs_diff(*b))
        .collect::<Vec<_>>();

    let diff_sum: u64 = diff.into_iter().map(|a| a as u64).sum();
    diff_sum as f64 / (img1.width() * img1.height() * 3) as f64 <= tol
}
