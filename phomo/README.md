# Phomo

> Photo mosaic generation library.

## Usage

To build a photo mosaic, you'll need a master image as well a bunch of so called tile images. The tile images will be arranged to best reconstruct the master image.
The master image is initialized with the [`Master`] `struct`.

The tile images can be loaded using some helper functions:

- [`read_images_from_dir`]
- [`read_images_from_dir_cropped`]
- [`read_images_from_dir_resized`]

The photo mosaic can be constructed using the [`Mosaic`] `struct`.

### Examples

I'll be using this 256x256 image as master image:

<img src="https://raw.githubusercontent.com/loiccoyle/phomo-rs/refs/heads/main/phomo/tests/data/master/master.png" alt="master.png" width="256" />

And for the tile images, we'll be using a susbset of the [UTKfaces](https://susanqq.github.io/UTKFace/).

#### Basic mosaic

Build a 12x12 photo mosaic.

```rust
use phomo::{Master, Mosaic, read_images_from_dir_cropped};

let master_file = "tests/data/master/master.png";
let grid_size = (16, 16);
let master = Master::from_file(master_file, grid_size).unwrap();

let tile_dir = "tests/data/mosaic/faces/";
let tiles = read_images_from_dir_cropped(tile_dir, master.cell_size.0, master.cell_size.1).unwrap();

let mosaic = Mosaic {master, tiles, grid_size};
let distance_matrix = mosaic.distance_matrix();

let mosaic_img = mosaic.build(distance_matrix);
```

<img src="https://raw.githubusercontent.com/loiccoyle/phomo-rs/refs/heads/main/phomo/tests/data/mosaic/mosaic_16_16.png" alt="mosaic.png" width="256" />

### Palette transfer - Tiles to master

Build a 8x8 photo mosaic, with palette transfer of the master image to the tile images. The color of the tiles will be adjusted to match the color distribution of the master image.

```rust
use phomo::{Master, Mosaic, read_images_from_dir_cropped, ColorMatch};

let master_file = "tests/data/master/master.png";
let grid_size = (16, 16);
let master = Master::from_file(master_file, grid_size).unwrap();

let tile_dir = "tests/data/mosaic/faces/";
let mut tiles = read_images_from_dir_cropped(tile_dir, master.cell_size.0, master.cell_size.1).unwrap();

tiles = tiles.match_palette(&master.img);

let mosaic = Mosaic {master, tiles, grid_size};
let distance_matrix = mosaic.distance_matrix();

let mosaic_img = mosaic.build(distance_matrix);
```

<img src="https://raw.githubusercontent.com/loiccoyle/phomo-rs/refs/heads/main/phomo/tests/data/mosaic/mosaic_16_16_match_tiles_to_master.png" alt="mosaic.png" width="256" />

### Palette transfer - Master to tiles

Build a 12x12 photo mosaic, with palette transfer of the tile images onto the master image. The color of the master image will be adjusted to match the color distribution of the tile images.

```rust
use image;
use phomo::{Master, Mosaic, read_images_from_dir_cropped, ColorMatch};

let master_file = "tests/data/master/master.png";
let grid_size = (16, 16);

let master_img = image::open(master_file).unwrap().to_rgb8();
let cell_size = (master_img.width() / grid_size.0, master_img.height() / grid_size.1);

let tile_dir = "tests/data/mosaic/faces/";
let tiles = read_images_from_dir_cropped(tile_dir, cell_size.0, cell_size.1).unwrap();

let master = Master::from_image(master_img.match_palette(&tiles), grid_size).unwrap();

let mosaic = Mosaic {master, tiles, grid_size};
let distance_matrix = mosaic.distance_matrix();

let mosaic_img = mosaic.build(distance_matrix);
```

<img src="https://raw.githubusercontent.com/loiccoyle/phomo-rs/refs/heads/main/phomo/tests/data/mosaic/mosaic_16_16_match_master_to_tiles.png" alt="mosaic.png" width="256" />

### Color Normalization

Build a 12x12 photo mosaic, with color distribution normalization. The colors of both the master image and tiles images will be adjusted to cover the full color space.

```rust
use image;
use phomo::{Master, Mosaic, read_images_from_dir_cropped, ColorMatch};

let master_file = "tests/data/master/master.png";
let grid_size = (16, 16);
let master_img = image::open(master_file).unwrap().to_rgb8();

let master = Master::from_image(master_img.equalize(), grid_size).unwrap();

let tile_dir = "tests/data/mosaic/faces/";
let mut tiles = read_images_from_dir_cropped(tile_dir, master.cell_size.0, master.cell_size.1).unwrap();
tiles = tiles.equalize();

let master = Master::from_image(master_img.match_palette(&tiles), grid_size).unwrap();

let mosaic = Mosaic {master, tiles, grid_size};
let distance_matrix = mosaic.distance_matrix();

let mosaic_img = mosaic.build(distance_matrix);
```

<img src="https://raw.githubusercontent.com/loiccoyle/phomo-rs/refs/heads/main/phomo/tests/data/mosaic/mosaic_16_16_equalized.png" alt="mosaic.png" width="256" />
