// generated by diplomat-tool
// dart format off

part of 'lib.g.dart';

final class RenamedNested2 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  RenamedNested2._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_namespace_Nested2_destroy));

}

@_DiplomatFfiUse('namespace_Nested2_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'namespace_Nested2_destroy')
// ignore: non_constant_identifier_names
external void _namespace_Nested2_destroy(ffi.Pointer<ffi.Void> self);

// dart format on
