// generated by diplomat-tool

part of 'lib.g.dart';

final class Float64Vec implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  Float64Vec._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_Float64Vec_destroy));

  factory Float64Vec.bool(core.List<bool> v) {
    final temp = _FinalizedArena();
    final result = _Float64Vec_new_bool(v._boolAllocIn(temp.arena));
    return Float64Vec._fromFfi(result, []);
  }

  factory Float64Vec.i16(core.List<int> v) {
    final temp = _FinalizedArena();
    final result = _Float64Vec_new_i16(v._int16AllocIn(temp.arena));
    return Float64Vec._fromFfi(result, []);
  }

  factory Float64Vec.u16(core.List<int> v) {
    final temp = _FinalizedArena();
    final result = _Float64Vec_new_u16(v._uint16AllocIn(temp.arena));
    return Float64Vec._fromFfi(result, []);
  }

  factory Float64Vec.isize(core.List<int> v) {
    final temp = _FinalizedArena();
    final result = _Float64Vec_new_isize(v._isizeAllocIn(temp.arena));
    return Float64Vec._fromFfi(result, []);
  }

  factory Float64Vec.usize(core.List<int> v) {
    final temp = _FinalizedArena();
    final result = _Float64Vec_new_usize(v._usizeAllocIn(temp.arena));
    return Float64Vec._fromFfi(result, []);
  }

  factory Float64Vec.f64BeBytes(ByteBuffer v) {
    final temp = _FinalizedArena();
    final result = _Float64Vec_new_f64_be_bytes(v.asUint8List()._uint8AllocIn(temp.arena));
    return Float64Vec._fromFfi(result, []);
  }

  factory Float64Vec(core.List<double> v) {
    final result = _Float64Vec_new_from_owned(v._float64AllocIn(_RustAlloc()));
    return Float64Vec._fromFfi(result, []);
  }

  core.List<double> get asSlice {
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> aEdges = [this];
    final result = _Float64Vec_as_slice(_ffi);
    return result._toDart(aEdges);
  }

  void fillSlice(core.List<double> v) {
    final temp = _FinalizedArena();
    _Float64Vec_fill_slice(_ffi, v._float64AllocIn(temp.arena));
  }

  void setValue(core.List<double> newSlice) {
    final temp = _FinalizedArena();
    _Float64Vec_set_value(_ffi, newSlice._float64AllocIn(temp.arena));
  }

  @override
  String toString() {
    final write = _Write();
    _Float64Vec_to_string(_ffi, write._ffi);
    return write.finalize();
  }

  core.List<double> borrow() {
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> aEdges = [this];
    final result = _Float64Vec_borrow(_ffi);
    return result._toDart(aEdges);
  }

  double? operator [](int i) {
    final result = _Float64Vec_get(_ffi, i);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok;
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'Float64Vec_destroy')
// ignore: non_constant_identifier_names
external void _Float64Vec_destroy(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(_SliceBool)>(isLeaf: true, symbol: 'Float64Vec_new_bool')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_bool(_SliceBool v);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(_SliceInt16)>(isLeaf: true, symbol: 'Float64Vec_new_i16')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_i16(_SliceInt16 v);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(_SliceUint16)>(isLeaf: true, symbol: 'Float64Vec_new_u16')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_u16(_SliceUint16 v);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(_SliceIsize)>(isLeaf: true, symbol: 'Float64Vec_new_isize')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_isize(_SliceIsize v);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(_SliceUsize)>(isLeaf: true, symbol: 'Float64Vec_new_usize')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_usize(_SliceUsize v);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(_SliceUint8)>(isLeaf: true, symbol: 'Float64Vec_new_f64_be_bytes')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_f64_be_bytes(_SliceUint8 v);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(_SliceDouble)>(isLeaf: true, symbol: 'Float64Vec_new_from_owned')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Float64Vec_new_from_owned(_SliceDouble v);

@meta.RecordUse()
@ffi.Native<_SliceDouble Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'Float64Vec_as_slice')
// ignore: non_constant_identifier_names
external _SliceDouble _Float64Vec_as_slice(ffi.Pointer<ffi.Opaque> self);

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, _SliceDouble)>(isLeaf: true, symbol: 'Float64Vec_fill_slice')
// ignore: non_constant_identifier_names
external void _Float64Vec_fill_slice(ffi.Pointer<ffi.Opaque> self, _SliceDouble v);

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, _SliceDouble)>(isLeaf: true, symbol: 'Float64Vec_set_value')
// ignore: non_constant_identifier_names
external void _Float64Vec_set_value(ffi.Pointer<ffi.Opaque> self, _SliceDouble newSlice);

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'Float64Vec_to_string')
// ignore: non_constant_identifier_names
external void _Float64Vec_to_string(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> write);

@meta.RecordUse()
@ffi.Native<_SliceDouble Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'Float64Vec_borrow')
// ignore: non_constant_identifier_names
external _SliceDouble _Float64Vec_borrow(ffi.Pointer<ffi.Opaque> self);

@meta.RecordUse()
@ffi.Native<_ResultDoubleVoid Function(ffi.Pointer<ffi.Opaque>, ffi.Size)>(isLeaf: true, symbol: 'Float64Vec_get')
// ignore: non_constant_identifier_names
external _ResultDoubleVoid _Float64Vec_get(ffi.Pointer<ffi.Opaque> self, int i);
