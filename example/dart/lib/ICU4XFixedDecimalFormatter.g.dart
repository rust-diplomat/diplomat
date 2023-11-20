// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
///
/// See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
final class ICU4XFixedDecimalFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XFixedDecimalFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XFixedDecimalFormatter_destroy'));

  /// Creates a new [`ICU4XFixedDecimalFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new) for more information.
  ///
  /// Throws [VoidError] on failure.
  factory ICU4XFixedDecimalFormatter.tryNew(ICU4XLocale locale, ICU4XDataProvider provider, ICU4XFixedDecimalFormatterOptions options) {
    final result = _ICU4XFixedDecimalFormatter_try_new(locale._underlying, provider._underlying, options._underlying);
    if (!result.isOk) {
      throw VoidError();
    }
    return ICU4XFixedDecimalFormatter._(result.union.ok);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XFixedDecimalFormatter_try_new =
    _capi<ffi.NativeFunction<_ResultOpaqueVoid Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, _ICU4XFixedDecimalFormatterOptionsFfi)>>('ICU4XFixedDecimalFormatter_try_new')
      .asFunction<_ResultOpaqueVoid Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, _ICU4XFixedDecimalFormatterOptionsFfi)>(isLeaf: true);

  /// Formats a [`ICU4XFixedDecimal`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.format) for more information.
  String formatWrite(ICU4XFixedDecimal value) {
    final writeable = _Writeable();
    _ICU4XFixedDecimalFormatter_format_write(_underlying, value._underlying, writeable._underlying);
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XFixedDecimalFormatter_format_write =
    _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('ICU4XFixedDecimalFormatter_format_write')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
