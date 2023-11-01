import 'dart:ffi' as ffi;
import 'dart:typed_data';
import 'diplomat_runtime.dart';
import 'package:ffi/ffi.dart' as allocators;

typedef Float64VecFfi = ffi.Pointer<ffi.Opaque>;

class Float64Vec implements ffi.Finalizable {
  final Float64VecFfi _underlying;

  factory Float64Vec.new(Float64List v) {
    final alloc = allocators.Arena();

    final vBytes = alloc.call<ffi.Double>(v.length);
    vBytes.asTypedList(v.length).setAll(0, v);

    final result = _newFfi(vBytes.cast(), v.length);
    alloc.releaseAll();
    return Float64VecFromFfi(result);
  }
  static late final _newFfi = capi<
          ffi.NativeFunction<
              Float64VecFfi Function(
                  ffi.Pointer<ffi.Double>, ffi.Size)>>('Float64Vec_new')
      .asFunction<Float64VecFfi Function(ffi.Pointer<ffi.Double>, int)>();

  void fillSlice(Float64List v) {
    final alloc = allocators.Arena();

    final vBytes = alloc.call<ffi.Double>(v.length);
    vBytes.asTypedList(v.length).setAll(0, v);

    _fillSliceFfi(this._underlying, vBytes.cast(), v.length);
    alloc.releaseAll();
  }

  static late final _fillSliceFfi = capi<
          ffi.NativeFunction<
              ffi.Void Function(Float64VecFfi, ffi.Pointer<ffi.Double>,
                  ffi.Size)>>('Float64Vec_fill_slice')
      .asFunction<void Function(Float64VecFfi, ffi.Pointer<ffi.Double>, int)>();

  void setValue(Float64List newSlice) {
    final alloc = allocators.Arena();

    final newSliceBytes = alloc.call<ffi.Double>(newSlice.length);
    newSliceBytes.asTypedList(newSlice.length).setAll(0, newSlice);

    _setValueFfi(this._underlying, newSliceBytes.cast(), newSlice.length);
    alloc.releaseAll();
  }

  static late final _setValueFfi = capi<
          ffi.NativeFunction<
              ffi.Void Function(Float64VecFfi, ffi.Pointer<ffi.Double>,
                  ffi.Size)>>('Float64Vec_set_value')
      .asFunction<void Function(Float64VecFfi, ffi.Pointer<ffi.Double>, int)>();

  Float64Vec._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'Float64Vec_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
Float64Vec Float64VecFromFfi(Float64VecFfi underlying) =>
    Float64Vec._(underlying);
Float64VecFfi Float64VecAsFfi(Float64Vec t) => t._underlying;
