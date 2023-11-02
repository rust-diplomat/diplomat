class _Slice extends ffi.Struct {
  external ffi.Pointer<ffi.Void> bytes;

  @ffi.Size()
  external int length;
}
