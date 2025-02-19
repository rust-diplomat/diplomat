final class _Write {
  final ffi.Pointer<ffi.Opaque> _ffi;

  _Write() : _ffi = _diplomat_buffer_write_create(0);
  
  String finalize() {
    try {
      final buf = _diplomat_buffer_write_get_bytes(_ffi);
      if (buf == ffi.Pointer.fromAddress(0)) {
        throw core.OutOfMemoryError();
      }
      return Utf8Decoder().convert(buf.asTypedList(_diplomat_buffer_write_len(_ffi)));
    } finally {
      _diplomat_buffer_write_destroy(_ffi);
    }
  }
}

@_DiplomatFfiUse('diplomat_buffer_write_create')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>(symbol: 'diplomat_buffer_write_create', isLeaf: true)
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _diplomat_buffer_write_create(int len);

@_DiplomatFfiUse('diplomat_buffer_write_len')
@ffi.Native<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_write_len', isLeaf: true)
// ignore: non_constant_identifier_names
external int _diplomat_buffer_write_len(ffi.Pointer<ffi.Opaque> ptr);

@_DiplomatFfiUse('diplomat_buffer_write_get_bytes')
@ffi.Native<ffi.Pointer<ffi.Uint8> Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_write_get_bytes', isLeaf: true)
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Uint8> _diplomat_buffer_write_get_bytes(ffi.Pointer<ffi.Opaque> ptr);

@_DiplomatFfiUse('diplomat_buffer_write_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_write_destroy', isLeaf: true)
// ignore: non_constant_identifier_names
external void _diplomat_buffer_write_destroy(ffi.Pointer<ffi.Opaque> ptr);