// generated by diplomat-tool

part of 'lib.g.dart';

final class OptionOpaque implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edgeSelf;

  // Internal constructor from FFI.
  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  OptionOpaque._(this._underlying, {core.List<Object> edgeSelf = const []}) : this._edgeSelf = edgeSelf {
    if (this._edgeSelf.isEmpty) {
      // Owned
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_OptionOpaque_destroy));

  static OptionOpaque? new_(int i) {
    final result = _OptionOpaque_new(i);
    return result.address == 0 ? null : OptionOpaque._(result);
  }

  static final OptionOpaque? none = () {
    final result = _OptionOpaque_new_none();
    return result.address == 0 ? null : OptionOpaque._(result);
  }();

  static final OptionStruct struct = () {
    final result = _OptionOpaque_new_struct();
    return OptionStruct._(result);
  }();

  static final OptionStruct structNones = () {
    final result = _OptionOpaque_new_struct_nones();
    return OptionStruct._(result);
  }();

  void assertInteger(int i) {
    _OptionOpaque_assert_integer(_underlying, i);
  }

  static bool optionOpaqueArgument(OptionOpaque? arg) {
    final result = _OptionOpaque_option_opaque_argument(arg?._underlying ?? ffi.Pointer.fromAddress(0));
    return result;
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'OptionOpaque_destroy')
// ignore: non_constant_identifier_names
external void _OptionOpaque_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Int32)>(isLeaf: true, symbol: 'OptionOpaque_new')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _OptionOpaque_new(int i);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'OptionOpaque_new_none')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _OptionOpaque_new_none();

@ffi.Native<_OptionStructFfi Function()>(isLeaf: true, symbol: 'OptionOpaque_new_struct')
// ignore: non_constant_identifier_names
external _OptionStructFfi _OptionOpaque_new_struct();

@ffi.Native<_OptionStructFfi Function()>(isLeaf: true, symbol: 'OptionOpaque_new_struct_nones')
// ignore: non_constant_identifier_names
external _OptionStructFfi _OptionOpaque_new_struct_nones();

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'OptionOpaque_assert_integer')
// ignore: non_constant_identifier_names
external void _OptionOpaque_assert_integer(ffi.Pointer<ffi.Opaque> self, int i);

@ffi.Native<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'OptionOpaque_option_opaque_argument')
// ignore: non_constant_identifier_names
external bool _OptionOpaque_option_opaque_argument(ffi.Pointer<ffi.Opaque> arg);
