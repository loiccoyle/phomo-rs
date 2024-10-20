# Changelog

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