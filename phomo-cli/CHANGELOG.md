# Changelog

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.3.0 to 0.3.1

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.7.0 to 0.7.1

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.7.1 to 0.7.2

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.7.2 to 0.7.3

## [0.6.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-cli-v0.5.0...phomo-cli-v0.6.0) (2025-01-26)


### Features

* add some more metrics ([00ff219](https://github.com/loiccoyle/phomo-rs/commit/00ff219adb1c318b1df21768ed53c6bf4f058fc6))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.6.0 to 0.7.0

## [0.5.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-cli-v0.4.0...phomo-cli-v0.5.0) (2025-01-25)


### ⚠ BREAKING CHANGES

* rm `--greedy`, use `--solver` with an enum to select assignment solver
* unify errors with `PhomoError` and modular solvers

### Features

* rm `--greedy`, use `--solver` with an enum to select assignment solver ([9a6547b](https://github.com/loiccoyle/phomo-rs/commit/9a6547b9cf6d8578d55ca590285f7d000cd9590d))


### Bug Fixes

* consistent solver init ([c80dd27](https://github.com/loiccoyle/phomo-rs/commit/c80dd27f0dbea06aa7558770f075bab329eadf6e))
* redo error handling with proper Error types ([5e7a1f6](https://github.com/loiccoyle/phomo-rs/commit/5e7a1f641b0a52e812c8f7523bd5e042e20a48ee))


### Code Refactoring

* unify errors with `PhomoError` and modular solvers ([40686a9](https://github.com/loiccoyle/phomo-rs/commit/40686a95026170e5cb481aedca6c29e53cb4504d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.5.0 to 0.6.0

## [0.4.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-cli-v0.3.0...phomo-cli-v0.4.0) (2025-01-21)


### ⚠ BREAKING CHANGES

* add `max_tile_occurrrences` as a field of the `Mosaic` struct

### Features

* **phomo-cli:** expose the greedy assignment algorithm in the CLI ([9e449f7](https://github.com/loiccoyle/phomo-rs/commit/9e449f7b2db760a71c30e783b3459d3aca7802fd))


### Code Refactoring

* add `max_tile_occurrrences` as a field of the `Mosaic` struct ([1894b10](https://github.com/loiccoyle/phomo-rs/commit/1894b10441aff5e7e88abe448593027e9cd2f443))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.4.0 to 0.5.0

## [0.3.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-cli-v0.2.4...phomo-cli-v0.3.0) (2025-01-20)


### Features

* **phomo:** repeated tiles ([ad5e6e7](https://github.com/loiccoyle/phomo-rs/commit/ad5e6e71c5ddfdb4b35703aa7781eb47ffa07183))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.3.2 to 0.4.0

## [0.2.4](https://github.com/loiccoyle/phomo-rs/compare/phomo-cli-v0.2.3...phomo-cli-v0.2.4) (2025-01-12)


### Bug Fixes

* **phomo:** update deps ([0511aaf](https://github.com/loiccoyle/phomo-rs/commit/0511aafcd9464f72fb1f0582c86d4a3e0f3eb142))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.3.1 to 0.3.2

## [0.2.2](https://github.com/loiccoyle/phomo-rs/compare/phomo-cli-v0.2.1...phomo-cli-v0.2.2) (2024-10-21)


### Bug Fixes

* use `crop_cover` in read dir method ([738ae43](https://github.com/loiccoyle/phomo-rs/commit/738ae43e88ce67d0c57b55d42701ac860e5fc769))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.2.0 to 0.3.0

## [0.2.1](https://github.com/loiccoyle/phomo-rs/compare/phomo-cli-v0.2.0...phomo-cli-v0.2.1) (2024-10-20)


### Bug Fixes

* bad field location ([3e64f0a](https://github.com/loiccoyle/phomo-rs/commit/3e64f0ad5f32c7ae9ba692e33ed6dc9260b23c52))

## [0.2.0](https://github.com/loiccoyle/phomo-rs/compare/phomo-cli-v0.1.0...phomo-cli-v0.2.0) (2024-10-20)


### Features

* add cli crate ([cba5785](https://github.com/loiccoyle/phomo-rs/commit/cba578542eaf1842e074676f9e7cba4f82f471ad))
* **phomo-cli:** add metric arg ([4fbe702](https://github.com/loiccoyle/phomo-rs/commit/4fbe702f01539b628f0d550636bec84d29350d79))


### Bug Fixes

* bad equalize implementation ([4315b41](https://github.com/loiccoyle/phomo-rs/commit/4315b412892aca75cfe98300447c0a5e590867b1))
* cleanup comment ([2cb6749](https://github.com/loiccoyle/phomo-rs/commit/2cb6749690a3167b7eb1e76da9f2e88f9d72a514))
* duplicate author ([ceabb45](https://github.com/loiccoyle/phomo-rs/commit/ceabb4554c12946fcc655e0eaf89396ee32e1a84))
* extra space ([7d34370](https://github.com/loiccoyle/phomo-rs/commit/7d34370eb556618bc22e7422674d19823d6cfd44))
* import order ([6448678](https://github.com/loiccoyle/phomo-rs/commit/6448678ce4b53ddf1045bf89cf1fac4d5a7b5e48))
* visibility warning ([166bdba](https://github.com/loiccoyle/phomo-rs/commit/166bdba91ddfb31493b7745b88febb6b957d0df3))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * phomo bumped from 0.1.0 to 0.2.0
