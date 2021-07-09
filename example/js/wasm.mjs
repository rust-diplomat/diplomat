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


if (typeof fetch === 'undefined') {
  const fs = await import("fs");
  const path = await import("path");
  const wasmFile = new Uint8Array(fs.readFileSync(path.resolve('../../target/wasm32-unknown-unknown/debug/example.wasm')));
  const loadedWasm = await WebAssembly.instantiate(wasmFile, imports);
  wasm = loadedWasm.instance.exports;
} else {
  const loadedWasm = await WebAssembly.instantiateStreaming(fetch('../../target/wasm32-unknown-unknown/debug/example.wasm'), imports);
  wasm = loadedWasm.instance.exports;
}

wasm.diplomat_init();

export default wasm;
