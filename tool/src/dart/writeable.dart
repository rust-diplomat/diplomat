class _Writeable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  _Writeable() : _underlying = _create(0);
  static late final _create =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>>(
              "diplomat_buffer_writeable_create")
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>();

  String toString() {
    final string = Utf8Decoder(allowMalformed: false)
        .convert(_get_bytes(_underlying).cast<ffi.Uint8>().asTypedList(_len(_underlying)));
    _destroy(_underlying);
    return string;
  }
  static late final _len =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              "diplomat_buffer_writeable_len")
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
  static late final _get_bytes = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Opaque>)>>(
          "diplomat_buffer_writeable_get_bytes")
      .asFunction<ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
  static late final _destroy =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              "diplomat_buffer_writeable_destroy")
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
