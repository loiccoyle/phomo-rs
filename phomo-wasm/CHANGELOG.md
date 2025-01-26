# Changelog

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.3.1 to 0.3.2

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.7.0 to 0.7.1

## [0.9.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-wasm-v0.8.1...phomo-wasm-v0.9.0) (2025-01-26)


### Features

* add some more metrics ([00ff219](https://github.com/loiccoyle/phomo-rs/commit/00ff219adb1c318b1df21768ed53c6bf4f058fc6))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.6.0 to 0.7.0

## [0.8.1](https://github.com/loiccoyle/phomo-rs/compare/phomo-wasm-v0.8.0...phomo-wasm-v0.8.1) (2025-01-25)


### Bug Fixes

* cleanup interface & add docs ([e28e460](https://github.com/loiccoyle/phomo-rs/commit/e28e460d1110d813a5e7c6432fc9c18c96225a0c))

## [0.8.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-wasm-v0.7.0...phomo-wasm-v0.8.0) (2025-01-25)


### Features

* add `build_blueprint_auction` function ([bccb781](https://github.com/loiccoyle/phomo-rs/commit/bccb781f56c3e7d8ef3af00868696655064d1ced))

## [0.7.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-wasm-v0.6.0...phomo-wasm-v0.7.0) (2025-01-25)


### ⚠ BREAKING CHANGES

* unify errors with `PhomoError` and modular solvers

### Features

* add `build_auction` to use the Auction based solver ([0a956b7](https://github.com/loiccoyle/phomo-rs/commit/0a956b760063ccc09995a4b383f49d076d525a25))


### Code Refactoring

* unify errors with `PhomoError` and modular solvers ([40686a9](https://github.com/loiccoyle/phomo-rs/commit/40686a95026170e5cb481aedca6c29e53cb4504d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.5.0 to 0.6.0

## [0.6.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-wasm-v0.5.0...phomo-wasm-v0.6.0) (2025-01-21)


### ⚠ BREAKING CHANGES

* add `max_tile_occurrrences` as a field of the `Mosaic` struct

### Features

* **phomo-wasm:** add greedy bindings ([5b7c114](https://github.com/loiccoyle/phomo-rs/commit/5b7c1148f174defb1e15b0a2fdccc3501d1eab3c))


### Code Refactoring

* add `max_tile_occurrrences` as a field of the `Mosaic` struct ([1894b10](https://github.com/loiccoyle/phomo-rs/commit/1894b10441aff5e7e88abe448593027e9cd2f443))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.4.0 to 0.5.0

## [0.5.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-wasm-v0.4.1...phomo-wasm-v0.5.0) (2025-01-20)


### Features

* **phomo:** repeated tiles ([ad5e6e7](https://github.com/loiccoyle/phomo-rs/commit/ad5e6e71c5ddfdb4b35703aa7781eb47ffa07183))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.3.2 to 0.4.0

## [0.4.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-wasm-v0.3.0...phomo-wasm-v0.4.0) (2024-10-24)


### Features

* add master image resize ([2d4f42b](https://github.com/loiccoyle/phomo-rs/commit/2d4f42b1b7fe48b82dfc9a933d783726c161f84a))

## [0.3.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-wasm-v0.2.1...phomo-wasm-v0.3.0) (2024-10-21)


### Features

* add master and tile getters ([ba49e53](https://github.com/loiccoyle/phomo-rs/commit/ba49e53431d358e3f8888c9147acb6673b2f7568))


### Bug Fixes

* update the master cell regions on match ([edf9f83](https://github.com/loiccoyle/phomo-rs/commit/edf9f83a36fa3dcab4ce49ddaa3b3f73516835f8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.3.0 to 0.3.1

## [0.2.1](https://github.com/loiccoyle/phomo-rs/compare/phomo-wasm-v0.2.0...phomo-wasm-v0.2.1) (2024-10-21)


### Bug Fixes

* use the crop to cover method ([ebd39a0](https://github.com/loiccoyle/phomo-rs/commit/ebd39a09c6501132747642ada394943e59ad6aa8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.2.0 to 0.3.0

## [0.2.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-wasm-v0.1.0...phomo-wasm-v0.2.0) (2024-10-20)


### Features

* add `blueprint` feature ([1c5bf3d](https://github.com/loiccoyle/phomo-rs/commit/1c5bf3d20071b7968e13f41560172d95493e7bf2))
* add `overlay_grid` method ([f82ff49](https://github.com/loiccoyle/phomo-rs/commit/f82ff4912c784712af28a64d912293391140744a))
* add `phomo-wasm` crate ([48542f3](https://github.com/loiccoyle/phomo-rs/commit/48542f3fb88beeeb2ffda2420c27b64b4bddd209))
* build blueprint ([1c5bf3d](https://github.com/loiccoyle/phomo-rs/commit/1c5bf3d20071b7968e13f41560172d95493e7bf2))
* **wasm:** add `wasm-logger` and panic on large crop ([32b6835](https://github.com/loiccoyle/phomo-rs/commit/32b6835a9ca252d3928240df4dfbf6855faea9d0))
* **wasm:** add resizing control, fix color matching ([30c1937](https://github.com/loiccoyle/phomo-rs/commit/30c1937e8828223910beea43cb424514a58e4e41))


### Bug Fixes

* don't duplicate data, and fix palette matching ([67ee355](https://github.com/loiccoyle/phomo-rs/commit/67ee355f5e3294007dbd0075468b2cfceae3d26a))
* use the inner master image, it could have been cropped to fit the grid ([5ff47f5](https://github.com/loiccoyle/phomo-rs/commit/5ff47f53c8b50f7b44af5354874e57bdb097a06a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.1.0 to 0.2.0
