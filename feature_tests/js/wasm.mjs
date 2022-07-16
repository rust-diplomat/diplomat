let wasm;

function readString(ptr) {
  const view = new Uint8Array(wasm.memory.buffer);
  let end = ptr;
  while (view[end]) end++;
  return (new TextDecoder("utf-8")).decode(view.subarray(ptr, end));
}

const imports = {
  env: {
    trace_js(ptr) {
      throw new Error(readString(ptr));
    }
  }
}

const path = await import('path');
const url = await import('url');
const dir = path.join(path.dirname(url.fileURLToPath(import.meta.url)), "diplomat_feature_tests.wasm");

if (typeof fetch === 'undefined') {
  const fs = await import("fs");
  const wasmFile = new Uint8Array(fs.readFileSync(dir));
  const loadedWasm = await WebAssembly.instantiate(wasmFile, imports);
  wasm = loadedWasm.instance.exports;
} else {
  const loadedWasm = await WebAssembly.instantiateStreaming(fetch(dir), imports);
  wasm = loadedWasm.instance.exports;
}

wasm.diplomat_init();

export default wasm;

