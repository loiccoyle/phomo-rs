use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phomo::read_images_from_dir_resized;
use phomo::solvers::auction::Auction;
use phomo::Greedy;
use phomo::Hungarian;
use phomo::Mosaic;
use phomo::Solve;
use phomo::SolverConfig;
use phomo::{norm_l1, norm_l2};
use std::path::PathBuf;
use std::time::Duration;

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

pub fn master_file() -> PathBuf {
    // 256x256
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("master")
        .join("master.png")
}

fn create_mosaic() -> Mosaic {
    let tile_dir = tile_dir();
    let master_file = master_file();

    let tile_imgs =
        read_images_from_dir_resized(tile_dir, 32, 32, image::imageops::FilterType::Nearest)
            .unwrap();
    let master_img = image::open(master_file).unwrap().to_rgb8();

    let result = Mosaic::from_images(master_img, tile_imgs, (8, 8));
    assert!(result.is_ok());
    result.unwrap()
}

fn open_master_img() -> image::RgbImage {
    image::open(master_file()).unwrap().to_rgb8()
}

fn bench_metrics(c: &mut Criterion) {
    let img1 = open_master_img();
    let img2 = img1.clone();

    let mut group = c.benchmark_group("metrics");
    group.bench_function("norm_l1", |b| {
        b.iter(|| black_box(norm_l1(&img1, &img2)));
    });
    group.bench_function("norm_l2", |b| {
        b.iter(|| black_box(norm_l2(&img1, &img2)));
    });
    group.finish();
}

fn bench_distance_matrix(c: &mut Criterion) {
    let mosaic = create_mosaic();
    c.bench_function("distance_matrix", |b| {
        b.iter(|| black_box(mosaic.distance_matrix()));
    });
}

fn bench_solvers(c: &mut Criterion) {
    let mosaic = create_mosaic();
    let distance_matrix = mosaic.distance_matrix();

    let mut group = c.benchmark_group("solvers");
    group.bench_function("hungarian", |b| {
        b.iter(|| black_box(Hungarian::default().solve(&distance_matrix)));
    });
    group.bench_function("greedy", |b| {
        b.iter(|| black_box(Greedy::default().solve(&distance_matrix)));
    });
    group.bench_function("auction", |b| {
        b.iter(|| black_box(Auction::default()).solve(&distance_matrix));
    });
    group.finish();
}

fn bench_build_mosaic(c: &mut Criterion) {
    let mosaic = create_mosaic();
    let distance_matrix = mosaic.distance_matrix();
    c.bench_function("build_mosaic", |b| {
        b.iter(|| {
            let result = black_box(mosaic.build(distance_matrix.clone(), SolverConfig::default()));
            assert!(result.is_ok());
        });
    });
}

fn bench_render(c: &mut Criterion) {
    let mosaic = create_mosaic();
    let distance_matrix = mosaic.distance_matrix();
    let assignments = distance_matrix.assignments(&mut Greedy::default()).unwrap();
    c.bench_function("render", |b| {
        b.iter(|| {
            let result = black_box(mosaic.render(assignments.clone()));
            assert!(result.is_ok());
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(10));
    targets = bench_distance_matrix, bench_solvers, bench_build_mosaic, bench_metrics, bench_render
}
criterion_main!(benches);
