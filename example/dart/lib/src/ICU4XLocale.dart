import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:typed_data';
import 'diplomat_runtime.dart';
import 'package:ffi/ffi.dart' as allocators;

typedef ICU4XLocaleFfi = ffi.Pointer<ffi.Opaque>;

/// An ICU4X Locale, capable of representing strings like `"en-US"`.
///
/// See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
class ICU4XLocale implements ffi.Finalizable {
  final ICU4XLocaleFfi _underlying;

  /// Construct an [`ICU4XLocale`] from a locale identifier represented as a string.
  factory ICU4XLocale.new(String name) {
    final alloc = allocators.Arena();

    final nameList = Utf8Encoder().convert(name);
    final nameBytes = alloc.call<ffi.Char>(nameList.length);
    nameBytes
        .cast<ffi.Uint8>()
        .asTypedList(nameList.length)
        .setAll(0, nameList);

    final result = _newFfi(nameBytes.cast(), nameList.length);
    alloc.releaseAll();
    return ICU4XLocaleFromFfi(result);
  }
  static late final _newFfi = capi<
          ffi.NativeFunction<
              ICU4XLocaleFfi Function(
                  ffi.Pointer<ffi.Char>, ffi.Size)>>('ICU4XLocale_new')
      .asFunction<ICU4XLocaleFfi Function(ffi.Pointer<ffi.Char>, int)>();

  /// Construct an [`ICU4XLocale`] from a locale identifier represented as bytes.
  factory ICU4XLocale.newFromBytes(Uint8List bytes) {
    final alloc = allocators.Arena();

    final bytesBytes = alloc.call<ffi.Uint8>(bytes.length);
    bytesBytes.asTypedList(bytes.length).setAll(0, bytes);

    final result = _newFromBytesFfi(bytesBytes.cast(), bytes.length);
    alloc.releaseAll();
    return ICU4XLocaleFromFfi(result);
  }
  static late final _newFromBytesFfi = capi<
          ffi.NativeFunction<
              ICU4XLocaleFfi Function(ffi.Pointer<ffi.Uint8>,
                  ffi.Size)>>('ICU4XLocale_new_from_bytes')
      .asFunction<ICU4XLocaleFfi Function(ffi.Pointer<ffi.Uint8>, int)>();

  ICU4XLocale._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'ICU4XLocale_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
ICU4XLocale ICU4XLocaleFromFfi(ICU4XLocaleFfi underlying) =>
    ICU4XLocale._(underlying);
ICU4XLocaleFfi ICU4XLocaleAsFfi(ICU4XLocale t) => t._underlying;
