import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef AttrOpaque1Ffi = ffi.Pointer<ffi.Opaque>;

class AttrOpaque1 implements ffi.Finalizable {
  final AttrOpaque1Ffi _underlying;

  void method() {
    _methodFfi(this._underlying);
  }

  static late final _methodFfi =
      capi<ffi.NativeFunction<ffi.Void Function(AttrOpaque1Ffi)>>(
              'AttrOpaque1_method')
          .asFunction<void Function(AttrOpaque1Ffi)>();

  void methodDisabledcpp() {
    _methodDisabledcppFfi(this._underlying);
  }

  static late final _methodDisabledcppFfi =
      capi<ffi.NativeFunction<ffi.Void Function(AttrOpaque1Ffi)>>(
              'AttrOpaque1_method_disabledcpp')
          .asFunction<void Function(AttrOpaque1Ffi)>();

  AttrOpaque1._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'AttrOpaque1_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
AttrOpaque1 AttrOpaque1FromFfi(AttrOpaque1Ffi underlying) =>
    AttrOpaque1._(underlying);
AttrOpaque1Ffi AttrOpaque1AsFfi(AttrOpaque1 t) => t._underlying;
