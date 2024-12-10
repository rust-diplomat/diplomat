// generated by diplomat-tool

part of 'lib.g.dart';

final class OpaqueMutexedString implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  OpaqueMutexedString._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_OpaqueMutexedString_destroy));

  static OpaqueMutexedString fromUsize(int number) {
    final result = _OpaqueMutexedString_from_usize(number);
    return OpaqueMutexedString._fromFfi(result, []);
  }

  void change(int number) {
    _OpaqueMutexedString_change(_ffi, number);
  }

  OpaqueMutexedString borrow() {
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> aEdges = [this];
    final result = _OpaqueMutexedString_borrow(_ffi);
    return OpaqueMutexedString._fromFfi(result, aEdges);
  }

  static OpaqueMutexedString borrowOther(OpaqueMutexedString other) {
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> aEdges = [other];
    final result = _OpaqueMutexedString_borrow_other(other._ffi);
    return OpaqueMutexedString._fromFfi(result, aEdges);
  }

  OpaqueMutexedString borrowSelfOrOther(OpaqueMutexedString other) {
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> aEdges = [this, other];
    final result = _OpaqueMutexedString_borrow_self_or_other(_ffi, other._ffi);
    return OpaqueMutexedString._fromFfi(result, aEdges);
  }

  int getLenAndAdd(int other) {
    final result = _OpaqueMutexedString_get_len_and_add(_ffi, other);
    return result;
  }

  String dummyStr() {
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> aEdges = [this];
    final result = _OpaqueMutexedString_dummy_str(_ffi);
    return result._toDart(aEdges);
  }

  Utf16Wrap wrapper() {
    final result = _OpaqueMutexedString_wrapper(_ffi);
    return Utf16Wrap._fromFfi(result, []);
  }

  int toUnsignedFromUnsigned(int input) {
    final result = _OpaqueMutexedString_to_unsigned_from_unsigned(_ffi, input);
    return result;
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'OpaqueMutexedString_destroy')
// ignore: non_constant_identifier_names
external void _OpaqueMutexedString_destroy(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>(isLeaf: true, symbol: 'OpaqueMutexedString_from_usize')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _OpaqueMutexedString_from_usize(int number);

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Size)>(isLeaf: true, symbol: 'OpaqueMutexedString_change')
// ignore: non_constant_identifier_names
external void _OpaqueMutexedString_change(ffi.Pointer<ffi.Opaque> self, int number);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'OpaqueMutexedString_borrow')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _OpaqueMutexedString_borrow(ffi.Pointer<ffi.Opaque> self);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'OpaqueMutexedString_borrow_other')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _OpaqueMutexedString_borrow_other(ffi.Pointer<ffi.Opaque> other);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'OpaqueMutexedString_borrow_self_or_other')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _OpaqueMutexedString_borrow_self_or_other(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> other);

@meta.RecordUse()
@ffi.Native<ffi.Size Function(ffi.Pointer<ffi.Opaque>, ffi.Size)>(isLeaf: true, symbol: 'OpaqueMutexedString_get_len_and_add')
// ignore: non_constant_identifier_names
external int _OpaqueMutexedString_get_len_and_add(ffi.Pointer<ffi.Opaque> self, int other);

@meta.RecordUse()
@ffi.Native<_SliceUtf8 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'OpaqueMutexedString_dummy_str')
// ignore: non_constant_identifier_names
external _SliceUtf8 _OpaqueMutexedString_dummy_str(ffi.Pointer<ffi.Opaque> self);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'OpaqueMutexedString_wrapper')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _OpaqueMutexedString_wrapper(ffi.Pointer<ffi.Opaque> self);

@meta.RecordUse()
@ffi.Native<ffi.Uint16 Function(ffi.Pointer<ffi.Opaque>, ffi.Uint16)>(isLeaf: true, symbol: 'OpaqueMutexedString_to_unsigned_from_unsigned')
// ignore: non_constant_identifier_names
external int _OpaqueMutexedString_to_unsigned_from_unsigned(ffi.Pointer<ffi.Opaque> self, int input);
