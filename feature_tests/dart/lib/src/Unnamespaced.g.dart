// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class Unnamespaced implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;

  // Internal constructor from FFI.
  // isOwned is whether this is owned (has finalizer) or not
  // This also takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  Unnamespaced._(this._underlying, bool isOwned, this._edge_self) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_namespace_Unnamespaced_destroy));

  factory Unnamespaced.make(AttrEnum e) {
    final result = _namespace_Unnamespaced_make(e.index);
    return Unnamespaced._(result, true, []);
  }

  void useNamespaced(AttrOpaque1 n) {
    _namespace_Unnamespaced_use_namespaced(_underlying, n._underlying);
  }
}

@meta.ResourceIdentifier()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'namespace_Unnamespaced_destroy')
// ignore: non_constant_identifier_names
external void _namespace_Unnamespaced_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Int32)>(isLeaf: true, symbol: 'namespace_Unnamespaced_make')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _namespace_Unnamespaced_make(int e);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'namespace_Unnamespaced_use_namespaced')
// ignore: non_constant_identifier_names
external void _namespace_Unnamespaced_use_namespaced(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> n);
