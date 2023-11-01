import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef AttrOpaque2Ffi = ffi.Pointer<ffi.Opaque>;

class AttrOpaque2 implements ffi.Finalizable {
  final AttrOpaque2Ffi _underlying;

  AttrOpaque2._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'AttrOpaque2_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
AttrOpaque2 AttrOpaque2FromFfi(AttrOpaque2Ffi underlying) =>
    AttrOpaque2._(underlying);
AttrOpaque2Ffi AttrOpaque2AsFfi(AttrOpaque2 t) => t._underlying;
