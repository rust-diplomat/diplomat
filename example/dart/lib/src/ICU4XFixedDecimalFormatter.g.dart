// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
///
/// See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
final class ICU4XFixedDecimalFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;

  ICU4XFixedDecimalFormatter._(this._underlying, bool isOwned, this._edge_self) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XFixedDecimalFormatter_destroy));

  /// Creates a new [`ICU4XFixedDecimalFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new) for more information.
  ///
  /// Throws [VoidError] on failure.
  factory ICU4XFixedDecimalFormatter(ICU4XLocale locale, ICU4XDataProvider provider, ICU4XFixedDecimalFormatterOptions options) {
    final temp = ffi2.Arena();
    final result = _ICU4XFixedDecimalFormatter_try_new(locale._underlying, provider._underlying, options._pointer(temp));
    temp.releaseAll();
    if (!result.isOk) {
      throw VoidError();
    }
    return ICU4XFixedDecimalFormatter._(result.union.ok, true, []);
  }

  /// Formats a [`ICU4XFixedDecimal`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.format) for more information.
  String formatWrite(ICU4XFixedDecimal value) {
    final writeable = _Writeable();
    _ICU4XFixedDecimalFormatter_format_write(_underlying, value._underlying, writeable._underlying);
    return writeable.finalize();
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XFixedDecimalFormatter_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XFixedDecimalFormatter_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_ResultOpaqueVoid Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, _ICU4XFixedDecimalFormatterOptionsFfi)>(isLeaf: true, symbol: 'ICU4XFixedDecimalFormatter_try_new')
// ignore: non_constant_identifier_names
external _ResultOpaqueVoid _ICU4XFixedDecimalFormatter_try_new(ffi.Pointer<ffi.Opaque> locale, ffi.Pointer<ffi.Opaque> provider, _ICU4XFixedDecimalFormatterOptionsFfi options);

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XFixedDecimalFormatter_format_write')
// ignore: non_constant_identifier_names
external void _ICU4XFixedDecimalFormatter_format_write(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> writeable);
