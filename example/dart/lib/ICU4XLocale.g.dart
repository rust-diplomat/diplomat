// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An ICU4X Locale, capable of representing strings like `"en-US"`.
///
/// See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
final class ICU4XLocale implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLocale._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XLocale_destroy'));

  /// Construct an [`ICU4XLocale`] from a locale identifier represented as a string.
  factory ICU4XLocale(String name) {
    final alloc = ffi2.Arena();
    final nameSlice = _SliceFfi2Utf8._fromDart(name, alloc);
    final result = _ICU4XLocale_new(nameSlice._bytes, nameSlice._length);
    alloc.releaseAll();
    return ICU4XLocale._(result);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_new =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi2.Utf8>, ffi.Size)>>('ICU4XLocale_new')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Construct an [`ICU4XLocale`] from a locale identifier represented as bytes.
  factory ICU4XLocale.fromBytes(Uint8List bytes) {
    final alloc = ffi2.Arena();
    final bytesSlice = _SliceFfiUint8._fromDart(bytes, alloc);
    final result = _ICU4XLocale_new_from_bytes(bytesSlice._bytes, bytesSlice._length);
    alloc.releaseAll();
    return ICU4XLocale._(result);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_new_from_bytes =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XLocale_new_from_bytes')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);
}
