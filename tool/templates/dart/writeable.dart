final class _Writeable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  _Writeable() : _underlying = _diplomat_buffer_writeable_create(0);
  
  String finalize() {
    final string = Utf8Decoder().convert(_diplomat_buffer_writeable_get_bytes(_underlying).asTypedList(_diplomat_buffer_writeable_len(_underlying)));
    _diplomat_buffer_writeable_destroy(_underlying);
    return string;
  }
}

  
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>(symbol: 'diplomat_buffer_writeable_create', isLeaf: true)
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _diplomat_buffer_writeable_create(int len);

@ffi.Native<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_writeable_len', isLeaf: true)
// ignore: non_constant_identifier_names
external int _diplomat_buffer_writeable_len(ffi.Pointer<ffi.Opaque> ptr);

@ffi.Native<ffi.Pointer<ffi.Uint8> Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_writeable_get_bytes', isLeaf: true)
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Uint8> _diplomat_buffer_writeable_get_bytes(ffi.Pointer<ffi.Opaque> ptr);

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_writeable_destroy', isLeaf: true)
// ignore: non_constant_identifier_names
external void _diplomat_buffer_writeable_destroy(ffi.Pointer<ffi.Opaque> ptr);