let wasm;

if (typeof fetch === 'undefined') {
  const fs = await import("fs");
  const path = await import("path");
  const wasmFile = new Uint8Array(fs.readFileSync(path.resolve('../../target/wasm32-unknown-unknown/debug/example.wasm')));
  const loadedWasm = await WebAssembly.instantiate(wasmFile);
  wasm = loadedWasm.instance.exports;
} else {
  const loadedWasm = await WebAssembly.instantiateStreaming(fetch('../../target/wasm32-unknown-unknown/debug/example.wasm'));
  wasm = loadedWasm.instance.exports;
}

export default wasm;
