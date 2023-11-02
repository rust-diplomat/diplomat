late final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
    _capi;

void init(String path) {
  _capi = ffi.DynamicLibrary.open(path).lookup;
}