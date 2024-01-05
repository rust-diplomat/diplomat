// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class RefList implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  RefList._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_RefList_destroy));

  factory RefList.node(RefListParameter data) {
    final result = _RefList_node(data._underlying);
    return RefList._(result);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'RefList_destroy')
// ignore: non_constant_identifier_names
external void _RefList_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'RefList_node')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _RefList_node(ffi.Pointer<ffi.Opaque> data);
