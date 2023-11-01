import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef ICU4XFixedDecimalFfi = ffi.Pointer<ffi.Opaque>;

/// See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
class ICU4XFixedDecimal implements ffi.Finalizable {
  final ICU4XFixedDecimalFfi _underlying;

  /// Construct an [`ICU4XFixedDecimal`] from an integer.
  factory ICU4XFixedDecimal.new(int v) {
    final result = _newFfi(v);
    return ICU4XFixedDecimalFromFfi(result);
  }
  static late final _newFfi =
      capi<ffi.NativeFunction<ICU4XFixedDecimalFfi Function(ffi.Int32)>>(
              'ICU4XFixedDecimal_new')
          .asFunction<ICU4XFixedDecimalFfi Function(int)>();

  /// Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
  ///
  /// See the [Rust documentation for `multiply_pow10`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
  void multiplyPow10(int power) {
    _multiplyPow10Ffi(this._underlying, power);
  }

  static late final _multiplyPow10Ffi = capi<
          ffi.NativeFunction<
              ffi.Void Function(ICU4XFixedDecimalFfi,
                  ffi.Int16)>>('ICU4XFixedDecimal_multiply_pow10')
      .asFunction<void Function(ICU4XFixedDecimalFfi, int)>();

  /// Format the [`ICU4XFixedDecimal`] as a string.
  ///
  /// See the [Rust documentation for `write_to`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.write_to) for more information.
  String toString() {
    final writeable = create_writeable();
    final result = _toStringFfi(this._underlying, writeable);
    return result.isOk ? writeable_to_string(writeable) : throw VoidError();
  }

  static late final _toStringFfi = capi<
          ffi.NativeFunction<
              ResultVoidUnionVoid Function(ICU4XFixedDecimalFfi,
                  DiplomatWriteable)>>('ICU4XFixedDecimal_to_string')
      .asFunction<
          ResultVoidUnionVoid Function(
              ICU4XFixedDecimalFfi, DiplomatWriteable)>();

  ICU4XFixedDecimal._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'ICU4XFixedDecimal_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
ICU4XFixedDecimal ICU4XFixedDecimalFromFfi(ICU4XFixedDecimalFfi underlying) =>
    ICU4XFixedDecimal._(underlying);
ICU4XFixedDecimalFfi ICU4XFixedDecimalAsFfi(ICU4XFixedDecimal t) =>
    t._underlying;

class ResultVoidUnionVoid extends ffi.Struct {
  @ffi.Bool()
  external bool isOk;
}
