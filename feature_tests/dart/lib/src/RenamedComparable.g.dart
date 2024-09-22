// generated by diplomat-tool

part of 'lib.g.dart';

final class RenamedComparable implements ffi.Finalizable, core.Comparable<RenamedComparable> {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  RenamedComparable._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_namespace_Comparable_destroy));

  static RenamedComparable new_(int int) {
    final result = _namespace_Comparable_new(int);
    return RenamedComparable._fromFfi(result, []);
  }

  int compareTo(RenamedComparable other) {
    final result = _namespace_Comparable_cmp(_ffi, other._ffi);
    return result;
  }

  @override
  bool operator ==(Object other) => other is RenamedComparable && compareTo(other) == 0;
  @override
  int get hashCode => 42; // Cannot get hash from Rust, so a constant is the only correct impl
}

@meta.RecordUse('namespace_Comparable_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'namespace_Comparable_destroy')
// ignore: non_constant_identifier_names
external void _namespace_Comparable_destroy(ffi.Pointer<ffi.Void> self);

@meta.RecordUse('namespace_Comparable_new')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Uint8)>(isLeaf: true, symbol: 'namespace_Comparable_new')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _namespace_Comparable_new(int int);

@meta.RecordUse('namespace_Comparable_cmp')
@ffi.Native<ffi.Int8 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'namespace_Comparable_cmp')
// ignore: non_constant_identifier_names
external int _namespace_Comparable_cmp(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> other);
