import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef TwoFfi = ffi.Pointer<ffi.Opaque>;

class Two implements ffi.Finalizable {
  final TwoFfi _underlying;

  Two._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'Two_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
Two TwoFromFfi(TwoFfi underlying) => Two._(underlying);
TwoFfi TwoAsFfi(Two t) => t._underlying;
