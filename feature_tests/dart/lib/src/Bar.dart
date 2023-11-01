import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef BarFfi = ffi.Pointer<ffi.Opaque>;

class Bar implements ffi.Finalizable {
  final BarFfi _underlying;

  Bar._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'Bar_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
Bar BarFromFfi(BarFfi underlying) => Bar._(underlying);
BarFfi BarAsFfi(Bar t) => t._underlying;
