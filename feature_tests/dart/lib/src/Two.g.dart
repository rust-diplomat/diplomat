// generated by diplomat-tool
// dart format off

part of 'lib.g.dart';

final class Two implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;
  // ignore: unused_field
  final core.List<Object> _aEdge;
  // ignore: unused_field
  final core.List<Object> _bEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  Two._fromFfi(this._ffi, this._selfEdge, this._aEdge, this._bEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_Two_destroy));

}

@_DiplomatFfiUse('Two_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'Two_destroy')
// ignore: non_constant_identifier_names
external void _Two_destroy(ffi.Pointer<ffi.Void> self);

// dart format on
