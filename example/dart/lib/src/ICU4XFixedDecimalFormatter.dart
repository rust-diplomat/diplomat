import 'ICU4XDataProvider.dart';
import 'ICU4XFixedDecimal.dart';
import 'ICU4XFixedDecimalFormatterOptions.dart';
import 'ICU4XLocale.dart';
import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef ICU4XFixedDecimalFormatterFfi = ffi.Pointer<ffi.Opaque>;

/// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
///
/// See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
class ICU4XFixedDecimalFormatter implements ffi.Finalizable {
  final ICU4XFixedDecimalFormatterFfi _underlying;

  /// Creates a new [`ICU4XFixedDecimalFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new) for more information.
  factory ICU4XFixedDecimalFormatter.tryNew(ICU4XLocale locale,
      ICU4XDataProvider provider, ICU4XFixedDecimalFormatterOptions options) {
    final result = _tryNewFfi(
        ICU4XLocaleAsFfi(locale),
        ICU4XDataProviderAsFfi(provider),
        ICU4XFixedDecimalFormatterOptionsAsFfi(options));
    return result.isOk
        ? ICU4XFixedDecimalFormatterFromFfi(result.union.ok)
        : throw VoidError();
  }
  static late final _tryNewFfi = capi<
              ffi.NativeFunction<
                  ResultICU4XFixedDecimalFormatterUnionVoid Function(
                      ICU4XLocaleFfi,
                      ICU4XDataProviderFfi,
                      ICU4XFixedDecimalFormatterOptionsFfi)>>(
          'ICU4XFixedDecimalFormatter_try_new')
      .asFunction<
          ResultICU4XFixedDecimalFormatterUnionVoid Function(ICU4XLocaleFfi,
              ICU4XDataProviderFfi, ICU4XFixedDecimalFormatterOptionsFfi)>();

  /// Formats a [`ICU4XFixedDecimal`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.format) for more information.
  String formatWrite(ICU4XFixedDecimal value) {
    final writeable = create_writeable();
    _formatWriteFfi(this._underlying, ICU4XFixedDecimalAsFfi(value), writeable);
    return writeable_to_string(writeable);
  }

  static late final _formatWriteFfi = capi<
              ffi.NativeFunction<
                  ffi.Void Function(ICU4XFixedDecimalFormatterFfi,
                      ICU4XFixedDecimalFfi, DiplomatWriteable)>>(
          'ICU4XFixedDecimalFormatter_format_write')
      .asFunction<
          void Function(ICU4XFixedDecimalFormatterFfi, ICU4XFixedDecimalFfi,
              DiplomatWriteable)>();

  ICU4XFixedDecimalFormatter._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'ICU4XFixedDecimalFormatter_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
ICU4XFixedDecimalFormatter ICU4XFixedDecimalFormatterFromFfi(
        ICU4XFixedDecimalFormatterFfi underlying) =>
    ICU4XFixedDecimalFormatter._(underlying);
ICU4XFixedDecimalFormatterFfi ICU4XFixedDecimalFormatterAsFfi(
        ICU4XFixedDecimalFormatter t) =>
    t._underlying;

class ICU4XFixedDecimalFormatterUnionVoid extends ffi.Union {
  external ICU4XFixedDecimalFormatterFfi ok;
}

class ResultICU4XFixedDecimalFormatterUnionVoid extends ffi.Struct {
  external ICU4XFixedDecimalFormatterUnionVoid union;

  @ffi.Bool()
  external bool isOk;
}
