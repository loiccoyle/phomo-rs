# Changelog

## [0.7.3](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.7.2...phomo-v0.7.3) (2025-02-01)


### Bug Fixes

* cleanup logging ([2ec638b](https://github.com/loiccoyle/phomo-rs/commit/2ec638bce4e35780d857004ac6778120727563fd))

## [0.7.2](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.7.1...phomo-v0.7.2) (2025-02-01)


### Bug Fixes

* unused import when build wasm ([2b9e5a2](https://github.com/loiccoyle/phomo-rs/commit/2b9e5a26963cc4f6a29ffe3339dd28090665c460))

## [0.7.1](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.7.0...phomo-v0.7.1) (2025-01-26)


### Bug Fixes

* bad column check ([b0599f6](https://github.com/loiccoyle/phomo-rs/commit/b0599f6d644e6b7216bfa7de89cf02a66149cece))

## [0.7.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.6.0...phomo-v0.7.0) (2025-01-26)


### Features

* add some more metrics ([00ff219](https://github.com/loiccoyle/phomo-rs/commit/00ff219adb1c318b1df21768ed53c6bf4f058fc6))


### Bug Fixes

* minor cleanup ([581b625](https://github.com/loiccoyle/phomo-rs/commit/581b62592d6c3883d4f6e72913aca5ad7d97fab8))


### Performance Improvements

* **auction:** get best & second best in one pass ([16634b8](https://github.com/loiccoyle/phomo-rs/commit/16634b80a4a33e56c4b33f6d89cadea5443be144))

## [0.6.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.5.0...phomo-v0.6.0) (2025-01-25)


### ⚠ BREAKING CHANGES

* unify errors with `PhomoError` and modular solvers

### Features

* add an Auction algorithm based solver ([e572a6c](https://github.com/loiccoyle/phomo-rs/commit/e572a6cab293e2f99a12b28679e19644f919d2af))


### Bug Fixes

* add logging when building blueprints ([c3a2132](https://github.com/loiccoyle/phomo-rs/commit/c3a2132048fafd6b26c2eb6ca536a26fb891eb2c))
* better lsap error handling ([9e00007](https://github.com/loiccoyle/phomo-rs/commit/9e00007a9ef19e46baf8e0a9ce968633e1b00bca))
* cleaner feature import handling ([0bc8df2](https://github.com/loiccoyle/phomo-rs/commit/0bc8df2edfc58f51e45c25ed781f94a23f68c6b7))
* consistent solver init ([c80dd27](https://github.com/loiccoyle/phomo-rs/commit/c80dd27f0dbea06aa7558770f075bab329eadf6e))
* redo error handling with proper Error types ([5e7a1f6](https://github.com/loiccoyle/phomo-rs/commit/5e7a1f641b0a52e812c8f7523bd5e042e20a48ee))


### Performance Improvements

* improve the Greedy solver performance ([53f1b73](https://github.com/loiccoyle/phomo-rs/commit/53f1b73d4b1da76253e1c12a9c79312f05ebf125))


### Code Refactoring

* unify errors with `PhomoError` and modular solvers ([40686a9](https://github.com/loiccoyle/phomo-rs/commit/40686a95026170e5cb481aedca6c29e53cb4504d))

## [0.5.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.4.0...phomo-v0.5.0) (2025-01-21)


### ⚠ BREAKING CHANGES

* add `max_tile_occurrrences` as a field of the `Mosaic` struct

### Features

* add a greedy tile assignment algorithm ([9a6f39d](https://github.com/loiccoyle/phomo-rs/commit/9a6f39dd54bcd4525e13bde72b3d263e0b3624f6))
* **phomo:** add greedy blueprint building ([275253a](https://github.com/loiccoyle/phomo-rs/commit/275253afe51513a60ee22c5d7f0b71914e887da9))


### Bug Fixes

* rm unused function ([cbfecdb](https://github.com/loiccoyle/phomo-rs/commit/cbfecdbac95ba86122b9f72d882c436798566f61))


### Performance Improvements

* **phomo:** initialize data structures with appropriate capacity ([735581d](https://github.com/loiccoyle/phomo-rs/commit/735581d258f579b861905ad2543671b16e991095))


### Code Refactoring

* add `max_tile_occurrrences` as a field of the `Mosaic` struct ([1894b10](https://github.com/loiccoyle/phomo-rs/commit/1894b10441aff5e7e88abe448593027e9cd2f443))

## [0.4.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.3.2...phomo-v0.4.0) (2025-01-20)


### Features

* **phomo:** repeated tiles ([ad5e6e7](https://github.com/loiccoyle/phomo-rs/commit/ad5e6e71c5ddfdb4b35703aa7781eb47ffa07183))


### Bug Fixes

* unused import when building wasm package ([2572f79](https://github.com/loiccoyle/phomo-rs/commit/2572f7976f3e2fa7556307314ddf7c06f381e94f))

## [0.3.2](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.3.1...phomo-v0.3.2) (2025-01-12)


### Bug Fixes

* **phomo:** update deps ([0511aaf](https://github.com/loiccoyle/phomo-rs/commit/0511aafcd9464f72fb1f0582c86d4a3e0f3eb142))
* rm old lock file ([9cb7b15](https://github.com/loiccoyle/phomo-rs/commit/9cb7b154e9c819b8bfa225f894645aab497d9a2e))

## [0.3.1](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.3.0...phomo-v0.3.1) (2024-10-21)


### Bug Fixes

* make `grid_size` public ([f5063b5](https://github.com/loiccoyle/phomo-rs/commit/f5063b5ab05150630dcbab100ece0dd5febc5f3a))

## [0.3.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.2.0...phomo-v0.3.0) (2024-10-21)


### Features

* add a utils method to crop along the largest dimension then resize ([cf190b9](https://github.com/loiccoyle/phomo-rs/commit/cf190b9abb1b0e8b82cb8221d65b5d34ad1dab12))


### Bug Fixes

* use `crop_cover` in read dir method ([738ae43](https://github.com/loiccoyle/phomo-rs/commit/738ae43e88ce67d0c57b55d42701ac860e5fc769))

## [0.2.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-v0.1.0...phomo-v0.2.0) (2024-10-20)


### Features

* add `blueprint` feature ([1c5bf3d](https://github.com/loiccoyle/phomo-rs/commit/1c5bf3d20071b7968e13f41560172d95493e7bf2))
* add grid overlay img generation ([66ccb4f](https://github.com/loiccoyle/phomo-rs/commit/66ccb4fd94c2e26636ca05d4d21a69187cc08771))
* build blueprint ([1c5bf3d](https://github.com/loiccoyle/phomo-rs/commit/1c5bf3d20071b7968e13f41560172d95493e7bf2))
* **phomo:** add optional `progress_bar` feature ([5b98d07](https://github.com/loiccoyle/phomo-rs/commit/5b98d07934322e279ecf03b73081aefd966244b9))


### Bug Fixes

* add a `warn` when cropping larger than the image ([cc19ef4](https://github.com/loiccoyle/phomo-rs/commit/cc19ef411a37278405e440f373aecb0f90fdf06e))
* bad equalize implementation ([4315b41](https://github.com/loiccoyle/phomo-rs/commit/4315b412892aca75cfe98300447c0a5e590867b1))
* check for `blueprint` feature ([8c458f3](https://github.com/loiccoyle/phomo-rs/commit/8c458f38ffd4359bd67c81270c6210c19d2a2316))
* **phomo:** bad tile coords ([70f2315](https://github.com/loiccoyle/phomo-rs/commit/70f2315c1d2580a11aef00b933feb6a491353655))
* public `utils::crop_imm_centered` ([8e593eb](https://github.com/loiccoyle/phomo-rs/commit/8e593eb462422f9e195c3bba3a94a190f7be198a))
* timing not working in wasm ([1a6a6eb](https://github.com/loiccoyle/phomo-rs/commit/1a6a6ebac75c544f99f0d950f1dc3a38ec544b4d))
* typo ([5054819](https://github.com/loiccoyle/phomo-rs/commit/5054819891b52a3ae9bd555f16028284aa7f1f10))
