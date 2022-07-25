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

const url = new URL('./diplomat-lib.wasm', import.meta.url);
if (typeof fetch === 'undefined') { // Node
  const fs = await import("fs");
  const wasmFile = new Uint8Array(fs.readFileSync(url));
  const loadedWasm = await WebAssembly.instantiate(wasmFile, imports);
  wasm = loadedWasm.instance.exports;
} else { // Browser
  const loadedWasm = await WebAssembly.instantiateStreaming(fetch(url), imports);
  wasm = loadedWasm.instance.exports;
}

wasm.diplomat_init();

export default wasm;
