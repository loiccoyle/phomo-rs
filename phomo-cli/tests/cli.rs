extern crate assert_cmd;
extern crate assert_fs;

use std::{
    env,
    path::{Path, PathBuf},
};

fn test_data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
}

fn master_img_file() -> PathBuf {
    test_data_dir().join("master.png")
}

fn tile_dir() -> PathBuf {
    test_data_dir().join("faces")
}

fn check_expected<P: AsRef<Path>, Q: AsRef<Path>>(img: P, expected: Q) -> bool {
    if env::var("PHOMO_UPDATE_EXPECTED").is_ok() {
        // copy the img to be the new expected img
        let _ = std::fs::copy(&img, &expected);
        return true;
    }

    image::open(img).unwrap().to_rgb8() == image::open(expected).unwrap().to_rgb8()
}

#[test]
fn help() {
    let mut cmd = assert_cmd::Command::cargo_bin("phomo").unwrap();
    cmd.arg("--help");

    cmd.assert().success();
}

#[test]
fn build_mosaic_cropped() {
    let output_file = assert_fs::NamedTempFile::new("output.png").unwrap();
    let expected_file = test_data_dir().join("mosaic_cropped_tiles.png");

    let mut cmd = assert_cmd::Command::cargo_bin("phomo").unwrap();
    cmd.arg(master_img_file().to_str().unwrap());
    cmd.arg(tile_dir().to_str().unwrap());
    cmd.arg(output_file.path().to_str().unwrap());
    cmd.arg("--crop-tiles");

    cmd.assert().success();
    assert!(output_file.path().exists());
    assert!(check_expected(output_file.path(), expected_file));
}

#[test]
fn build_mosaic_resized() {
    let output_file = assert_fs::NamedTempFile::new("output.png").unwrap();
    let expected_file = test_data_dir().join("mosaic_resized_tiles.png");

    let mut cmd = assert_cmd::Command::cargo_bin("phomo").unwrap();
    cmd.arg(master_img_file().to_str().unwrap());
    cmd.arg(tile_dir().to_str().unwrap());
    cmd.arg(output_file.path().to_str().unwrap());
    cmd.arg("--resize-tiles");

    cmd.assert().success();
    assert!(output_file.path().exists());
    assert!(check_expected(output_file.path(), expected_file));
}

#[test]
fn build_mosaic_repeats() {
    let output_file = assert_fs::NamedTempFile::new("output.png").unwrap();
    let expected_file = test_data_dir().join("mosaic_repeats.png");

    let mut cmd = assert_cmd::Command::cargo_bin("phomo").unwrap();
    cmd.arg(master_img_file().to_str().unwrap());
    cmd.arg(tile_dir().to_str().unwrap());
    cmd.arg(output_file.path().to_str().unwrap());
    cmd.arg("--resize-tiles");
    cmd.arg("--n-appearances=2");

    cmd.assert().success();
    assert!(output_file.path().exists());
    assert!(check_expected(output_file.path(), expected_file));
}

#[test]
fn build_mosaic_equalized() {
    let output_file = assert_fs::NamedTempFile::new("output.png").unwrap();
    let expected_file = test_data_dir().join("mosaic_equalized.png");

    let mut cmd = assert_cmd::Command::cargo_bin("phomo").unwrap();
    cmd.arg(master_img_file().to_str().unwrap());
    cmd.arg(tile_dir().to_str().unwrap());
    cmd.arg(output_file.path().to_str().unwrap());
    cmd.arg("--resize-tiles");
    cmd.arg("--equalize");

    cmd.assert().success();
    assert!(output_file.path().exists());
    assert!(check_expected(output_file.path(), expected_file));
}

#[test]
fn build_mosaic_transfer_tiles_to_master() {
    let output_file = assert_fs::NamedTempFile::new("output.png").unwrap();
    let expected_file = test_data_dir().join("mosaic_transfer_tiles_to_master.png");

    let mut cmd = assert_cmd::Command::cargo_bin("phomo").unwrap();
    cmd.arg(master_img_file().to_str().unwrap());
    cmd.arg(tile_dir().to_str().unwrap());
    cmd.arg(output_file.path().to_str().unwrap());
    cmd.arg("--resize-tiles");
    cmd.arg("--transfer-tiles-to-master");

    cmd.assert().success();
    assert!(output_file.path().exists());
    assert!(check_expected(output_file.path(), expected_file));
}

#[test]
fn build_mosaic_transfer_master_to_tiles() {
    let output_file = assert_fs::NamedTempFile::new("output.png").unwrap();
    let expected_file = test_data_dir().join("mosaic_transfer_master_to_tiles.png");

    let mut cmd = assert_cmd::Command::cargo_bin("phomo").unwrap();
    cmd.arg(master_img_file().to_str().unwrap());
    cmd.arg(tile_dir().to_str().unwrap());
    cmd.arg(output_file.path().to_str().unwrap());
    cmd.arg("--resize-tiles");
    cmd.arg("--transfer-master-to-tiles");

    cmd.assert().success();
    assert!(output_file.path().exists());
    assert!(check_expected(output_file.path(), expected_file));
}

#[test]
fn build_mosaic_grid_size() {
    let output_file = assert_fs::NamedTempFile::new("output.png").unwrap();
    let expected_file = test_data_dir().join("mosaic_10_10.png");

    let mut cmd = assert_cmd::Command::cargo_bin("phomo").unwrap();
    cmd.arg(master_img_file().to_str().unwrap());
    cmd.arg(tile_dir().to_str().unwrap());
    cmd.arg(output_file.path().to_str().unwrap());
    cmd.arg("-g 10,10");
    cmd.arg("--resize-tiles");

    cmd.assert().success();
    assert!(output_file.path().exists());
    assert!(check_expected(output_file.path(), expected_file));
}

#[test]
fn build_mosaic_bad_grid_size() {
    let output_file = assert_fs::NamedTempFile::new("output.png").unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("phomo").unwrap();
    cmd.arg(master_img_file().to_str().unwrap());
    cmd.arg(tile_dir().to_str().unwrap());
    cmd.arg(output_file.path().to_str().unwrap());
    cmd.arg("-g 100,100");

    cmd.assert().failure();
}

#[test]
fn build_mosaic_wrong_tile_size() {
    let output_file = assert_fs::NamedTempFile::new("output.png").unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("phomo").unwrap();
    cmd.arg(master_img_file().to_str().unwrap());
    cmd.arg(tile_dir().to_str().unwrap());
    cmd.arg(output_file.path().to_str().unwrap());

    cmd.assert().failure();
}
