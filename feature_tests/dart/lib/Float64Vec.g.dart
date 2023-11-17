// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

class Float64Vec implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Float64Vec._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('Float64Vec_destroy'));

  factory Float64Vec(Float64List v) {
    final alloc = ffi2.Arena();
    final vSlice = _SliceFfiDouble._fromDart(v, alloc);

    final result = _Float64Vec_new(vSlice._bytes, vSlice._length);
    alloc.releaseAll();
    return Float64Vec._(result);
  }
  // ignore: non_constant_identifier_names
  static final _Float64Vec_new = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Double>, ffi.Size)>>('Float64Vec_new')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Double>, int)>(isLeaf: true);

  void fillSlice(Float64List v) {
    final alloc = ffi2.Arena();
    final vSlice = _SliceFfiDouble._fromDart(v, alloc);

    _Float64Vec_fill_slice(_underlying, vSlice._bytes, vSlice._length);
    alloc.releaseAll();
  }

  // ignore: non_constant_identifier_names
  static final _Float64Vec_fill_slice = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Double>, ffi.Size)>>('Float64Vec_fill_slice')
      .asFunction<
          void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Double>,
              int)>(isLeaf: true);

  void setValue(Float64List newSlice) {
    final alloc = ffi2.Arena();
    final newSliceSlice = _SliceFfiDouble._fromDart(newSlice, alloc);

    _Float64Vec_set_value(
        _underlying, newSliceSlice._bytes, newSliceSlice._length);
    alloc.releaseAll();
  }

  // ignore: non_constant_identifier_names
  static final _Float64Vec_set_value = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Double>, ffi.Size)>>('Float64Vec_set_value')
      .asFunction<
          void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Double>,
              int)>(isLeaf: true);
}
