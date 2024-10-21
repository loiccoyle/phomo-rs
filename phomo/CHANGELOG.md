# Changelog

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
