<!-- <p align="center"><img src="https://i.imgur.com/4jvon2p.png" width="1000"></p> -->
<h1 align="center">phomo</h1>
<p align="center"><b>create photo mosaics</b></p>

<p align="center">
  <a href="https://crates.io/crates/phomo"><img src="https://img.shields.io/crates/v/phomo"></a>
  <a href="https://crates.io/crates/phomo-cli"><img src="https://img.shields.io/crates/v/phomo-cli"></a>
  <a href="https://npmjs.com/package/phomo-wasm"><img src="https://img.shields.io/npm/v/phomo-wasm"></a>
  <a href="https://docs.rs/phomo/latest/phomo/"><img src="https://img.shields.io/docsrs/phomo"></a>
  <a href="https://github.com/loiccoyle/phomo-rs/actions"><img src="https://github.com/loiccoyle/phomo-rs/actions/workflows/ci.yml/badge.svg"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-mit-blue.svg"></a>
  <img src="https://img.shields.io/badge/platform-linux%20%7c%20macos%20%7c%20windows-informational">
</p>

<p align="center">
  <b>
  Make your own photo mosaics <a href="https://loiccoyle.com/phomo-rs">here</a>
  </b>
</p>


This repo contains two crates:

- `phomo` crate contains the photo mosaic generation library.
- `phomo-cli` crate provides the command line interface to the `phomo` crate.

And one `npm` package:

- `phomo-wasm` provides the `wasm` bindings and allows `phomo` to be used in the browser.

## 📦 Installation

## Command line

To use the `phomo` binary to build photo mosaics, install the `phomo-cli` crate:

```sh
cargo install phomo-cli
```

### Usage

Once installed, you can use the `phomo` binary.

Something like:

```sh
phomo master_image.png tile_directory/ output_mosaic.png
```

If doubt see the help:

<!-- help start -->
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
