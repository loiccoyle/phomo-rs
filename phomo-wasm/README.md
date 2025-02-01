# phomo-wasm

> WASM bindings for the [`phomo`](https://github.com/loiccoyle/phomo-rs) photo mosaic library.

Demo site: <https://loiccoyle.com/phomo-rs>
Demo site source: <https://github.com/loiccoyle/phomo-rs/tree/gh-pages>

## Initialization

`phomo-wasm` uses [`wasm-bindgen-rayon`](https://github.com/RReverser/wasm-bindgen-rayon) to enable `rayon` parallelism in the browser.
This requires `phomo-wasm` to be packages with the [`web` target](https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target)
which builds the package as a ES module. As such, you'll need to initialize wasm yourself.

```js
import init, { initThreadPool } from "phomo-wasm";

await init();
await initThreadPool(navigator.hardwareConcurrency || 1);
```

I would recommend looking at the [demo site source code](https://github.com/loiccoyle/phomo-rs/tree/gh-pages) for usage with `vite`.
