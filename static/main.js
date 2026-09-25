// Bootstrap for the compiled WebAssembly module.
//
// The Makefile copies this file next to the generated `bangs.js` inside the
// versioned build directory (dist/bangs-<sha>/), so `./bangs.js` resolves to
// the JS glue wasm-bindgen generates, and its `init()` in turn fetches
// `./bangs_bg.wasm` relative to this directory.
import init, { start } from "./bangs.js";

init()
  .then(start)
  // Also surfaces the deliberate "Redirect to ..." error raised in debug
  // mode, which the original app threw as an uncaught exception.
  .catch(console.error);
