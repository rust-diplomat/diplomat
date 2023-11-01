import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef RefListParameterFfi = ffi.Pointer<ffi.Opaque>;

class RefListParameter implements ffi.Finalizable {
  final RefListParameterFfi _underlying;

  RefListParameter._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'RefListParameter_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
RefListParameter RefListParameterFromFfi(RefListParameterFfi underlying) =>
    RefListParameter._(underlying);
RefListParameterFfi RefListParameterAsFfi(RefListParameter t) => t._underlying;
