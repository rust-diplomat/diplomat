// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

class RefList implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  RefList._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('RefList_destroy'));

  factory RefList.node(RefListParameter data) {
    final result = _RefList_node(data._underlying);
    return RefList._(result);
  }
  // ignore: non_constant_identifier_names
  static final _RefList_node = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('RefList_node')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}
