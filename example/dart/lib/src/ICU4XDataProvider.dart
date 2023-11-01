import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef ICU4XDataProviderFfi = ffi.Pointer<ffi.Opaque>;

/// An ICU4X data provider, capable of loading ICU4X data keys from some source.
///
/// See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
class ICU4XDataProvider implements ffi.Finalizable {
  final ICU4XDataProviderFfi _underlying;

  /// See the [Rust documentation for `get_static_provider`](https://docs.rs/icu_testdata/latest/icu_testdata/fn.get_static_provider.html) for more information.
  factory ICU4XDataProvider.newStatic() {
    final result = _newStaticFfi();
    return ICU4XDataProviderFromFfi(result);
  }
  static late final _newStaticFfi =
      capi<ffi.NativeFunction<ICU4XDataProviderFfi Function()>>(
              'ICU4XDataProvider_new_static')
          .asFunction<ICU4XDataProviderFfi Function()>();

  /// This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
  static void returnsResult() {
    final result = _returnsResultFfi();
    if (!result.isOk) {
      throw VoidError();
    }
  }

  static late final _returnsResultFfi =
      capi<ffi.NativeFunction<ResultVoidUnionVoid Function()>>(
              'ICU4XDataProvider_returns_result')
          .asFunction<ResultVoidUnionVoid Function()>();

  ICU4XDataProvider._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'ICU4XDataProvider_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
ICU4XDataProvider ICU4XDataProviderFromFfi(ICU4XDataProviderFfi underlying) =>
    ICU4XDataProvider._(underlying);
ICU4XDataProviderFfi ICU4XDataProviderAsFfi(ICU4XDataProvider t) =>
    t._underlying;

class ResultVoidUnionVoid extends ffi.Struct {
  @ffi.Bool()
  external bool isOk;
}
