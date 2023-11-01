import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef OptionOpaqueCharFfi = ffi.Pointer<ffi.Opaque>;

class OptionOpaqueChar implements ffi.Finalizable {
  final OptionOpaqueCharFfi _underlying;

  void assertChar(int ch) {
    _assertCharFfi(this._underlying, ch);
  }

  static late final _assertCharFfi = capi<
          ffi.NativeFunction<
              ffi.Void Function(OptionOpaqueCharFfi,
                  ffi.Uint32)>>('OptionOpaqueChar_assert_char')
      .asFunction<void Function(OptionOpaqueCharFfi, int)>();

  OptionOpaqueChar._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'OptionOpaqueChar_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
OptionOpaqueChar OptionOpaqueCharFromFfi(OptionOpaqueCharFfi underlying) =>
    OptionOpaqueChar._(underlying);
OptionOpaqueCharFfi OptionOpaqueCharAsFfi(OptionOpaqueChar t) => t._underlying;
