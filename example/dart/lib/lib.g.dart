// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:typed_data';
import 'package:ffi/ffi.dart' as ffi2;

late final ffi.Pointer<T> Function<T extends ffi.NativeType>(String) _capi;
void init(String path) => _capi = ffi.DynamicLibrary.open(path).lookup;

final _callocFree = Finalizer(ffi2.calloc.free);

/// An ICU4X data provider, capable of loading ICU4X data keys from some source.
///
/// See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
class ICU4XDataProvider implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XDataProvider._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XDataProvider_destroy'));

  /// See the [Rust documentation for `get_static_provider`](https://docs.rs/icu_testdata/latest/icu_testdata/fn.get_static_provider.html) for more information.
  factory ICU4XDataProvider.static() {
    final result = _newStaticFfi();
    return ICU4XDataProvider._(result);
  }
  static final _newStaticFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XDataProvider_new_static')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
  static void returnsResult() {
    final result = _returnsResultFfi();
    if (!result.isOk) {
      throw VoidError();
    }
  }

  static final _returnsResultFfi =
      _capi<ffi.NativeFunction<_ResultVoidVoid Function()>>(
              'ICU4XDataProvider_returns_result')
          .asFunction<_ResultVoidVoid Function()>(isLeaf: true);
}

/// See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
class ICU4XFixedDecimal implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XFixedDecimal._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XFixedDecimal_destroy'));

  /// Construct an [`ICU4XFixedDecimal`] from an integer.
  factory ICU4XFixedDecimal(int v) {
    final result = _newFfi(v);
    return ICU4XFixedDecimal._(result);
  }
  static final _newFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Int32)>>(
              'ICU4XFixedDecimal_new')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);

  /// Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
  ///
  /// See the [Rust documentation for `multiply_pow10`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
  void multiplyPow10(int power) {
    _multiplyPow10Ffi(_underlying, power);
  }

  static final _multiplyPow10Ffi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_multiply_pow10')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Format the [`ICU4XFixedDecimal`] as a string.
  ///
  /// See the [Rust documentation for `write_to`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.write_to) for more information.
  @override
  String toString() {
    final writeable = _Writeable();
    final result = _toStringFfi(_underlying, writeable._underlying);
    return result.isOk ? writeable.finalize() : throw VoidError();
  }

  static final _toStringFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidVoid Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XFixedDecimal_to_string')
      .asFunction<
          _ResultVoidVoid Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
///
/// See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
class ICU4XFixedDecimalFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XFixedDecimalFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XFixedDecimalFormatter_destroy'));

  /// Creates a new [`ICU4XFixedDecimalFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new) for more information.
  factory ICU4XFixedDecimalFormatter.tryNew(ICU4XLocale locale,
      ICU4XDataProvider provider, ICU4XFixedDecimalFormatterOptions options) {
    final result = _tryNewFfi(
        locale._underlying, provider._underlying, options._underlying);
    return result.isOk
        ? ICU4XFixedDecimalFormatter._(result.union.ok)
        : throw VoidError();
  }
  static final _tryNewFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueVoid Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>,
                      _ICU4XFixedDecimalFormatterOptionsFfi)>>(
          'ICU4XFixedDecimalFormatter_try_new')
      .asFunction<
          _ResultOpaqueVoid Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              _ICU4XFixedDecimalFormatterOptionsFfi)>(isLeaf: true);

  /// Formats a [`ICU4XFixedDecimal`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.format) for more information.
  String formatWrite(ICU4XFixedDecimal value) {
    final writeable = _Writeable();
    _formatWriteFfi(_underlying, value._underlying, writeable._underlying);
    return writeable.finalize();
  }

  static final _formatWriteFfi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XFixedDecimalFormatter_format_write')
      .asFunction<
          void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

class _ICU4XFixedDecimalFormatterOptionsFfi extends ffi.Struct {
  @ffi.Uint32()
  external int groupingStrategy;
  @ffi.Bool()
  external bool someOtherConfig;
}

class ICU4XFixedDecimalFormatterOptions {
  final _ICU4XFixedDecimalFormatterOptionsFfi _underlying;

  // ignore: unused_element
  ICU4XFixedDecimalFormatterOptions._(this._underlying);

  factory ICU4XFixedDecimalFormatterOptions() {
    final pointer = ffi2.calloc<_ICU4XFixedDecimalFormatterOptionsFfi>();
    final result = ICU4XFixedDecimalFormatterOptions._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  ICU4XFixedDecimalGroupingStrategy get groupingStrategy =>
      ICU4XFixedDecimalGroupingStrategy._(_underlying.groupingStrategy);
  set groupingStrategy(ICU4XFixedDecimalGroupingStrategy groupingStrategy) {
    _underlying.groupingStrategy = groupingStrategy._id;
  }

  bool get someOtherConfig => _underlying.someOtherConfig;
  set someOtherConfig(bool someOtherConfig) {
    _underlying.someOtherConfig = someOtherConfig;
  }

  factory ICU4XFixedDecimalFormatterOptions() {
    final result = _defaultFfi();
    return ICU4XFixedDecimalFormatterOptions._(result);
  }
  static final _defaultFfi = _capi<
          ffi.NativeFunction<
              _ICU4XFixedDecimalFormatterOptionsFfi
                  Function()>>('ICU4XFixedDecimalFormatterOptions_default')
      .asFunction<_ICU4XFixedDecimalFormatterOptionsFfi Function()>(
          isLeaf: true);

  @override
  bool operator ==(Object other) =>
      other is ICU4XFixedDecimalFormatterOptions &&
      other._underlying.groupingStrategy == _underlying.groupingStrategy &&
      other._underlying.someOtherConfig == _underlying.someOtherConfig;

  @override
  int get hashCode => Object.hashAll([
        _underlying.groupingStrategy,
        _underlying.someOtherConfig,
      ]);
}

enum ICU4XFixedDecimalGroupingStrategy {
  /// Auto grouping
  auto.__(0),

  /// No grouping
  never.__(1),

  /// Always group
  always.__(2),

  /// At least 2 groups
  min2.__(3);

  const ICU4XFixedDecimalGroupingStrategy.__(this._id);

  // ignore: unused_element
  factory ICU4XFixedDecimalGroupingStrategy._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// An ICU4X Locale, capable of representing strings like `"en-US"`.
///
/// See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
class ICU4XLocale implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLocale._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XLocale_destroy'));

  /// Construct an [`ICU4XLocale`] from a locale identifier represented as a string.
  factory ICU4XLocale(String name) {
    final alloc = ffi2.Arena();
    final nameSlice = _SliceFfi2Utf8._fromDart(name, alloc);

    final result = _newFfi(nameSlice._bytes, nameSlice._length);
    alloc.releaseAll();
    return ICU4XLocale._(result);
  }
  static final _newFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi2.Utf8>, ffi.Size)>>('ICU4XLocale_new')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Construct an [`ICU4XLocale`] from a locale identifier represented as bytes.
  factory ICU4XLocale.fromBytes(Uint8List bytes) {
    final alloc = ffi2.Arena();
    final bytesSlice = _SliceFfiUint8._fromDart(bytes, alloc);

    final result = _newFromBytesFfi(bytesSlice._bytes, bytesSlice._length);
    alloc.releaseAll();
    return ICU4XLocale._(result);
  }
  static final _newFromBytesFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Uint8>,
                  ffi.Size)>>('ICU4XLocale_new_from_bytes')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);
}

class _ResultOpaqueVoidUnion extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> ok;
}

class _ResultOpaqueVoid extends ffi.Struct {
  external _ResultOpaqueVoidUnion union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultVoidVoid extends ffi.Struct {
  @ffi.Bool()
  external bool isOk;
}

class _SliceFfi2Utf8 extends ffi.Struct {
  external ffi.Pointer<ffi2.Utf8> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfi2Utf8 _fromDart(String value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfi2Utf8>();
    final slice = pointer.ref;
    final units = Utf8Encoder().convert(value);
    slice._length = units.length;
    slice._bytes = allocator<ffi.Uint8>(slice._length).cast();
    slice._bytes.cast<ffi.Uint8>().asTypedList(slice._length).setAll(0, units);

    return slice;
  }

  // ignore: unused_element
  String get _asDart =>
      Utf8Decoder().convert(_bytes.cast<ffi.Uint8>().asTypedList(_length));

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfi2Utf8 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes.cast<ffi.Uint8>()[i] != _bytes.cast<ffi.Uint8>()[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

class _SliceFfiUint8 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfiUint8 _fromDart(Uint8List value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfiUint8>();
    final slice = pointer.ref;
    slice._length = value.length;
    slice._bytes = allocator(slice._length);
    slice._bytes.asTypedList(slice._length).setAll(0, value);

    return slice;
  }

  // ignore: unused_element
  Uint8List get _asDart => _bytes.asTypedList(_length);

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfiUint8 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

/// An unspecified error value
class VoidError {}

class _Writeable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  _Writeable() : _underlying = _create(0);
  static final _create =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>>(
              'diplomat_buffer_writeable_create')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>();

  String finalize() {
    final string =
        _getBytes(_underlying).toDartString(length: _len(_underlying));
    _destroy(_underlying);
    return string;
  }

  static final _len =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'diplomat_buffer_writeable_len')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
  static final _getBytes = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi2.Utf8> Function(ffi.Pointer<ffi.Opaque>)>>(
          'diplomat_buffer_writeable_get_bytes')
      .asFunction<ffi.Pointer<ffi2.Utf8> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
  static final _destroy =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'diplomat_buffer_writeable_destroy')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
