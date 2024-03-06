// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
final class ICU4XFixedDecimal implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;

  // Internal constructor from FFI.
  // isOwned is whether this is owned (has finalizer) or not
  // This also takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  ICU4XFixedDecimal._(this._underlying, bool isOwned, this._edge_self) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XFixedDecimal_destroy));

  /// Construct an [`ICU4XFixedDecimal`] from an integer.
  factory ICU4XFixedDecimal(int v) {
    final result = _ICU4XFixedDecimal_new(v);
    return ICU4XFixedDecimal._(result, true, []);
  }

  /// Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
  ///
  /// See the [Rust documentation for `multiply_pow10`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
  void multiplyPow10(int power) {
    _ICU4XFixedDecimal_multiply_pow10(_underlying, power);
  }

  /// Format the [`ICU4XFixedDecimal`] as a string.
  ///
  /// See the [Rust documentation for `write_to`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.write_to) for more information.
  String? toStringFallible() {
    final writeable = _Writeable();
    final result = _ICU4XFixedDecimal_to_string(_underlying, writeable._underlying);
    if (!result.isOk) {
      return null;
    }
    return writeable.finalize();
  }
}

@meta.ResourceIdentifier()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XFixedDecimal_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XFixedDecimal_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Int32)>(isLeaf: true, symbol: 'ICU4XFixedDecimal_new')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XFixedDecimal_new(int v);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Int16)>(isLeaf: true, symbol: 'ICU4XFixedDecimal_multiply_pow10')
// ignore: non_constant_identifier_names
external void _ICU4XFixedDecimal_multiply_pow10(ffi.Pointer<ffi.Opaque> self, int power);

@meta.ResourceIdentifier()
@ffi.Native<_ResultVoidVoid Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XFixedDecimal_to_string')
// ignore: non_constant_identifier_names
external _ResultVoidVoid _ICU4XFixedDecimal_to_string(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> writeable);
