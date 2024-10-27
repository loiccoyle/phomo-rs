<p align="center"><img src="https://github.com/loiccoyle/phomo-rs/blob/main/assets/cover.png?raw=true" width="800"></p>
<p align="center"><b>Easy photo mosaics</b></p>
<p align="center">
  <b>
  Make your own photo mosaics <a href="https://loiccoyle.com/phomo-rs">here</a>!
  </b>
</p>

<p align="center">
  <a href="https://crates.io/crates/phomo"><img src="https://img.shields.io/crates/v/phomo"></a>
  <a href="https://crates.io/crates/phomo-cli"><img src="https://img.shields.io/crates/v/phomo-cli"></a>
  <a href="https://npmjs.com/package/phomo-wasm"><img src="https://img.shields.io/npm/v/phomo-wasm"></a>
  <a href="https://docs.rs/phomo/latest/phomo/"><img src="https://img.shields.io/docsrs/phomo"></a>
  <a href="https://aur.archlinux.org/packages/phomo-git"><img src="https://img.shields.io/aur/version/phomo-git"></a>
  <a href="https://github.com/loiccoyle/phomo-rs/actions"><img src="https://github.com/loiccoyle/phomo-rs/actions/workflows/ci.yml/badge.svg"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-mit-blue.svg"></a>
  <img src="https://img.shields.io/badge/platform-linux%20%7c%20macos%20%7c%20windows-informational">
</p>

> Prefer python? Check out [`loiccoyle/phomo`](https://github.com/loiccoyle/phomo)!

This repo contains two crates:

- `phomo` crate contains the photo mosaic generation library.
- `phomo-cli` crate provides the command line interface to the `phomo` crate.

And one `npm` package:

- `phomo-wasm` provides the `wasm` bindings and allows `phomo` to be used in the browser.

## 📦 Installation

## Command line

### Cargo

To use the `phomo` binary to build photo mosaics, install the `phomo-cli` crate:

```sh
cargo install phomo-cli
```

### Arch linux (AUR)

Using your favourite AUR helper, install the `phomo-git` package:

```sh
paru -S phomo-git
```

### Usage

Once installed, you can use the `phomo` binary.

Something like:

```sh
phomo master_image.png tile_directory/ output_mosaic.png
```

If doubt see the help:

<!-- help start -->

```console
$ phomo -h
Usage: phomo [OPTIONS] <MASTER_FILE> <TILE_DIR> <OUTPUT>

Arguments:
  <MASTER_FILE>  Master image
  <TILE_DIR>     Tile directory
  <OUTPUT>       Output mosaic file

Options:
  -g, --grid-size <WIDTH,HEIGHT>  Grid size, the number of tiles along the width and height
      --crop-tiles                Crop tiles to grid cell size
      --resize-tiles              Resize tiles to grid cell size
      --equalize                  Equalize the master and tile image color distributions
      --transfer-master-to-tiles  Transfer the color palette of the master image to the tile images
      --transfer-tiles-to-master  Transfer the color palette of the tile images to the master image
      --metric <METRIC>           The distance metric to use [default: norm-l1] [possible values: norm-l1, norm-l2]
  -v, --verbose...                Increase logging verbosity
  -q, --quiet...                  Decrease logging verbosity
  -h, --help                      Print help (see more with '--help')
  -V, --version                   Print version
```

<!-- help end -->

## Library

To use the library as a dependency in your projects, add the `phomo` crate:

```sh
cargo add phomo
```

See the [docs](https://docs.rs/phomo) for usage.

## Wasm

To use the `wasm` bindings in your project, add the `phomo-wasm` `npm` package:

```sh
npm add phomo-wasm
```

I would recommend taking a look at the [demo page's source code](https://github.com/loiccoyle/phomo-rs/tree/gh-pages).
