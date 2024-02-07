// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class AttrOpaque1 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edgeSelf;

  // Internal constructor from FFI.
  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  AttrOpaque1._(this._underlying, {core.List<Object> edgeSelf = const []}) : this._edgeSelf = edgeSelf {
    if (this._edgeSelf.isEmpty) {
      // Owned
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_AttrOpaque1_destroy));

  factory AttrOpaque1() {
    final result = _namespace_AttrOpaque1_new();
    return AttrOpaque1._(result);
  }

  int get method {
    final result = _namespace_AttrOpaque1_method(_underlying);
    return result;
  }

  int get abirenamed {
    final result = _renamed_on_abi_only(_underlying);
    return result;
  }

  void methodDisabledcpp() {
    _namespace_AttrOpaque1_method_disabledcpp(_underlying);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'AttrOpaque1_destroy')
// ignore: non_constant_identifier_names
external void _AttrOpaque1_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'namespace_AttrOpaque1_new')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _namespace_AttrOpaque1_new();

@ffi.Native<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'namespace_AttrOpaque1_method')
// ignore: non_constant_identifier_names
external int _namespace_AttrOpaque1_method(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'renamed_on_abi_only')
// ignore: non_constant_identifier_names
external int _renamed_on_abi_only(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'namespace_AttrOpaque1_method_disabledcpp')
// ignore: non_constant_identifier_names
external void _namespace_AttrOpaque1_method_disabledcpp(ffi.Pointer<ffi.Opaque> self);
