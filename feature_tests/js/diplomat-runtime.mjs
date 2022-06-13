function readString(wasm, ptr, len) {
  const buf = new Uint8Array(wasm.memory.buffer, ptr, len);
  return (new TextDecoder("utf-8")).decode(buf)
}

export function withWriteable(wasm, callback) {
  const writeable = wasm.diplomat_buffer_writeable_create(0);
  try {
    callback(writeable);
    const outStringPtr = wasm.diplomat_buffer_writeable_get_bytes(writeable);
    const outStringLen = wasm.diplomat_buffer_writeable_len(writeable);
    return readString(wasm, outStringPtr, outStringLen);
  } finally {
    wasm.diplomat_buffer_writeable_destroy(writeable);
  }
}

export class FFIError extends Error {
  constructor(error_value) {
    super("Error over FFI");
    this.error_value = error_value; // (2)
  }
}

export function extractCodePoint(str, param) {
  const cp = str.codePointAt?.(0);
  if ((!cp && cp !== 0) || [...str]?.length != 1) {
    throw new TypeError(`Expected single-character string for char parameter ${param}, found ${str}`);
  }
  return cp;
}

// Fixes(61)
export class RcAlloc {

  static str = (string) => {
    const align = 1;
    const bytes = (new TextEncoder()).encode(string);
    const size = bytes.length;
    const ptr = wasm.diplomat_alloc(size, align);
    (new Uint8Array(wasm.memory.buffer, ptr, size)).set(bytes, 0);
    return new RcAlloc(ptr, size, align);
  }

  static slice = (slice, align) => {
    const bytes = new Uint8Array(slice);
    const size = bytes.length;
    const ptr = wasm.diplomat_alloc(size, align);
    // is there a way to write the slice directly into the view?
    (new Uint8Array(wasm.memory.buffer, ptr, size)).set(bytes, 0);
    return new RcAlloc(ptr, size, align);
  }

  static alloc = (size, align) => {
    const ptr = wasm.diplomat_alloc(size, align);
    return new RcAlloc(ptr, size, align);
  }

  constructor(ptr, size, align) {
    this.ptr = ptr;
    this.size = size;
    this.align = align;

    RcAlloc_finalizer.register(this, { ptr, size, align });
  }

  free() {
    wasm.diplomat_free(this.ptr, this.size, this.align);

    // Unregister to prevent the double free
    RcAlloc_finalizer.unregister(this);

    // No longer own the ptr, so make `free` fail if called again
    delete this.ptr;
  }
}

const RcAlloc_finalizer = new FinalizationRegistry(({ ptr, size, align }) => {
  wasm.diplomat_free(ptr, size, align);
});