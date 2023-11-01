import 'dart:convert';
import 'dart:ffi' as ffi;

late final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
    capi;

void init(String path) {
  capi = ffi.DynamicLibrary.open(path).lookup;
}

typedef DiplomatWriteable = ffi.Pointer<ffi.Opaque>;

DiplomatWriteable create_writeable() => _create(0);
late final _create =
    capi<ffi.NativeFunction<DiplomatWriteable Function(ffi.Size)>>(
            "diplomat_buffer_writeable_create")
        .asFunction<DiplomatWriteable Function(int)>();

String writeable_to_string(DiplomatWriteable self) {
  final string = Utf8Decoder(allowMalformed: false)
      .convert(_get_bytes(self).cast<ffi.Uint8>().asTypedList(_len(self)));
  _destroy(self);
  return string;
}

late final _len =
    capi<ffi.NativeFunction<ffi.Size Function(DiplomatWriteable)>>(
            "diplomat_buffer_writeable_len")
        .asFunction<int Function(DiplomatWriteable)>();
late final _get_bytes =
    capi<ffi.NativeFunction<ffi.Pointer<ffi.Char> Function(DiplomatWriteable)>>(
            "diplomat_buffer_writeable_get_bytes")
        .asFunction<ffi.Pointer<ffi.Char> Function(DiplomatWriteable)>();
late final _destroy =
    capi<ffi.NativeFunction<ffi.Void Function(DiplomatWriteable)>>(
            "diplomat_buffer_writeable_destroy")
        .asFunction<void Function(DiplomatWriteable)>();

class Slice extends ffi.Struct {
  external ffi.Pointer<ffi.Void> bytes;

  @ffi.Size()
  external int length;
}

class VoidError {}