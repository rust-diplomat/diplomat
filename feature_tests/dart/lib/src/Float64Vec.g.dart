// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class Float64Vec implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Float64Vec._(this._underlying, bool isOwned) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_Float64Vec_destroy));

  factory Float64Vec(core.List<double> v) {
    final temp = ffi2.Arena();
    final vView = v.float64View;
    final result = _Float64Vec_new(vView.pointer(temp), vView.length);
    temp.releaseAll();
    return Float64Vec._(result, true);
  }

  factory Float64Vec.bool(core.List<bool> v) {
    final temp = ffi2.Arena();
    final vView = v.boolView;
    final result = _Float64Vec_new_bool(vView.pointer(temp), vView.length);
    temp.releaseAll();
    return Float64Vec._(result, true);
  }

  factory Float64Vec.i16(core.List<int> v) {
    final temp = ffi2.Arena();
    final vView = v.int16View;
    final result = _Float64Vec_new_i16(vView.pointer(temp), vView.length);
    temp.releaseAll();
    return Float64Vec._(result, true);
  }

  factory Float64Vec.u16(core.List<int> v) {
    final temp = ffi2.Arena();
    final vView = v.uint16View;
    final result = _Float64Vec_new_u16(vView.pointer(temp), vView.length);
    temp.releaseAll();
    return Float64Vec._(result, true);
  }

  factory Float64Vec.isize(core.List<int> v) {
    final temp = ffi2.Arena();
    final vView = v.isizeView;
    final result = _Float64Vec_new_isize(vView.pointer(temp), vView.length);
    temp.releaseAll();
    return Float64Vec._(result, true);
  }

  factory Float64Vec.usize(core.List<int> v) {
    final temp = ffi2.Arena();
    final vView = v.usizeView;
    final result = _Float64Vec_new_usize(vView.pointer(temp), vView.length);
    temp.releaseAll();
    return Float64Vec._(result, true);
  }

  factory Float64Vec.f64BeBytes(ByteBuffer v) {
    final temp = ffi2.Arena();
    final vView = v;
    final result = _Float64Vec_new_f64_be_bytes(vView.pointer(temp), vView.length);
    temp.releaseAll();
    return Float64Vec._(result, true);
  }

  void fillSlice(core.List<double> v) {
    final temp = ffi2.Arena();
    final vView = v.float64View;
    _Float64Vec_fill_slice(_underlying, vView.pointer(temp), vView.length);
    temp.releaseAll();
  }

  void setValue(core.List<double> newSlice) {
    final temp = ffi2.Arena();
    final newSliceView = newSlice.float64View;
    _Float64Vec_set_value(_underlying, newSliceView.pointer(temp), newSliceView.length);
    temp.releaseAll();
  }

  @override
  String toString() {
    final writeable = _Writeable();
    _Float64Vec_to_string(_underlying, writeable._underlying);
    return writeable.finalize();
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'Float64Vec_destroy')
// ignore: non_constant_identifier_names
external void _Float64Vec_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Double>, ffi.Size)>(isLeaf: true, symbol: 'Float64Vec_new')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new(ffi.Pointer<ffi.Double> vData, int vLength);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Bool>, ffi.Size)>(isLeaf: true, symbol: 'Float64Vec_new_bool')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_bool(ffi.Pointer<ffi.Bool> vData, int vLength);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Int16>, ffi.Size)>(isLeaf: true, symbol: 'Float64Vec_new_i16')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_i16(ffi.Pointer<ffi.Int16> vData, int vLength);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Uint16>, ffi.Size)>(isLeaf: true, symbol: 'Float64Vec_new_u16')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_u16(ffi.Pointer<ffi.Uint16> vData, int vLength);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.IntPtr>, ffi.Size)>(isLeaf: true, symbol: 'Float64Vec_new_isize')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_isize(ffi.Pointer<ffi.IntPtr> vData, int vLength);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Size>, ffi.Size)>(isLeaf: true, symbol: 'Float64Vec_new_usize')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_usize(ffi.Pointer<ffi.Size> vData, int vLength);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'Float64Vec_new_f64_be_bytes')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_f64_be_bytes(ffi.Pointer<ffi.Uint8> vData, int vLength);

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Double>, ffi.Size)>(isLeaf: true, symbol: 'Float64Vec_fill_slice')
// ignore: non_constant_identifier_names
external void _Float64Vec_fill_slice(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Double> vData, int vLength);

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Double>, ffi.Size)>(isLeaf: true, symbol: 'Float64Vec_set_value')
// ignore: non_constant_identifier_names
external void _Float64Vec_set_value(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Double> newSliceData, int newSliceLength);

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'Float64Vec_to_string')
// ignore: non_constant_identifier_names
external void _Float64Vec_to_string(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> writeable);
