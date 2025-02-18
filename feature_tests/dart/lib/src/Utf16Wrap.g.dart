// generated by diplomat-tool

part of 'lib.g.dart';

final class Utf16Wrap implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  Utf16Wrap._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_Utf16Wrap_destroy));

  factory Utf16Wrap(String input) {
    final temp = _FinalizedArena();
    final result = _Utf16Wrap_from_utf16(input._utf16AllocIn(temp.arena));
    return Utf16Wrap._fromFfi(result, []);
  }

  String getDebugStr() {
    final write = _Write();
    _Utf16Wrap_get_debug_str(_ffi, write._ffi);
    return write.finalize();
  }

  String borrowCont() {
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> aEdges = [this];
    final result = _Utf16Wrap_borrow_cont(_ffi);
    return result._toDart(aEdges);
  }
}

@_DiplomatFfiUse('Utf16Wrap_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'Utf16Wrap_destroy')
// ignore: non_constant_identifier_names
external void _Utf16Wrap_destroy(ffi.Pointer<ffi.Void> self);

@_DiplomatFfiUse('Utf16Wrap_from_utf16')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(_SliceUtf16)>(isLeaf: true, symbol: 'Utf16Wrap_from_utf16')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Utf16Wrap_from_utf16(_SliceUtf16 input);

@_DiplomatFfiUse('Utf16Wrap_get_debug_str')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'Utf16Wrap_get_debug_str')
// ignore: non_constant_identifier_names
external void _Utf16Wrap_get_debug_str(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> write);

@_DiplomatFfiUse('Utf16Wrap_borrow_cont')
@ffi.Native<_SliceUtf16 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'Utf16Wrap_borrow_cont')
// ignore: non_constant_identifier_names
external _SliceUtf16 _Utf16Wrap_borrow_cont(ffi.Pointer<ffi.Opaque> self);
