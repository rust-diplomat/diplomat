import 'OptionStruct.dart';
import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef OptionOpaqueFfi = ffi.Pointer<ffi.Opaque>;
class OptionOpaque implements ffi.Finalizable {

  final OptionOpaqueFfi _underlying;

  
static OptionOpaque? new(int i) { 
    final result = _newFfi(i);
    return result.address == 0 ? null : OptionOpaqueFromFfi(result);
  }
  static late final _newFfi = capi<ffi.NativeFunction<OptionOpaqueFfi Function(ffi.Int32)>>('OptionOpaque_new')
  .asFunction<OptionOpaqueFfi Function(int)>();

  
static OptionOpaque? newNone() { 
    final result = _newNoneFfi();
    return result.address == 0 ? null : OptionOpaqueFromFfi(result);
  }
  static late final _newNoneFfi = capi<ffi.NativeFunction<OptionOpaqueFfi Function()>>('OptionOpaque_new_none')
  .asFunction<OptionOpaqueFfi Function()>();

  
static OptionStruct newStruct() { 
    final result = _newStructFfi();
    return OptionStructFromFfi(result);
  }
  static late final _newStructFfi = capi<ffi.NativeFunction<OptionStructFfi Function()>>('OptionOpaque_new_struct')
  .asFunction<OptionStructFfi Function()>();

  
static OptionStruct newStructNones() { 
    final result = _newStructNonesFfi();
    return OptionStructFromFfi(result);
  }
  static late final _newStructNonesFfi = capi<ffi.NativeFunction<OptionStructFfi Function()>>('OptionOpaque_new_struct_nones')
  .asFunction<OptionStructFfi Function()>();

  
void assertInteger(int i) { 
    _assertIntegerFfi(this._underlying,i);
  }
  static late final _assertIntegerFfi = capi<ffi.NativeFunction<ffi.Void Function(OptionOpaqueFfi, ffi.Int32)>>('OptionOpaque_assert_integer')
  .asFunction<void Function(OptionOpaqueFfi, int)>();

  
static bool optionOpaqueArgument(OptionOpaque? arg) { 
    final result = _optionOpaqueArgumentFfi(OptionOpaque?AsFfi(arg));
    return result;
  }
  static late final _optionOpaqueArgumentFfi = capi<ffi.NativeFunction<ffi.Bool Function(OptionOpaqueFfi)>>('OptionOpaque_option_opaque_argument')
  .asFunction<bool Function(OptionOpaqueFfi)>();

  OptionOpaque._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>('OptionOpaque_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
OptionOpaque OptionOpaqueFromFfi(OptionOpaqueFfi underlying) => OptionOpaque._(underlying);
OptionOpaqueFfi OptionOpaqueAsFfi(OptionOpaque t) => t._underlying;

