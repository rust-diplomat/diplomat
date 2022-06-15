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

export class DiplomatBuf {

  static str = (wasm, string) => {
    const bytes = (new TextEncoder()).encode(string);
    return new DiplomatBuf(wasm, bytes, 1);
  }

  static slice = (wasm, slice, align) => {
    const bytes = new Uint8Array(slice);
    return new DiplomatBuf(wasm, bytes, align);
  }

  constructor(wasm, bytes, align) {
    const size = bytes.length;
    const ptr = wasm.diplomat_alloc(size, align);
    (new Uint8Array(wasm.memory.buffer, ptr, size)).set(bytes, 0);

    this.ptr = ptr;
    this.size = size;
    this.align = align;
    this.freed = false;

    DiplomatBuf_finalizer.register(this, { ptr, size, align });
  }

  free() {
    if (!freed) {
      this.freed = true;

      wasm.diplomat_free(this.ptr, this.size, this.align);

      // Unregister to prevent the double free
      DiplomatBuf_finalizer.unregister(this);
    }
  }
}

const DiplomatBuf_finalizer = new FinalizationRegistry(({ ptr, size, align }) => {
  wasm.diplomat_free(ptr, size, align);
});