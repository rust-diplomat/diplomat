// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class OptionOpaque implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  OptionOpaque._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('OptionOpaque_destroy'));

  static OptionOpaque? new_(int i) {
    final result = _OptionOpaque_new(i);
    return result.address == 0 ? null : OptionOpaque._(result);
  }

  // ignore: non_constant_identifier_names
  static final _OptionOpaque_new =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Int32)>>('OptionOpaque_new')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);

  static final OptionOpaque? none = () {
    final result = _OptionOpaque_new_none();
    return result.address == 0 ? null : OptionOpaque._(result);
  }();

  // ignore: non_constant_identifier_names
  static final _OptionOpaque_new_none =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>('OptionOpaque_new_none')
      .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  static final OptionStruct? returns = () {
    final result = _OptionOpaque_returns();
    if (!result.isOk) {
      return null;
    }
    return OptionStruct._(result.union.ok);
  }();

  // ignore: non_constant_identifier_names
  static final _OptionOpaque_returns =
    _capi<ffi.NativeFunction<_ResultOptionStructFfiVoid Function()>>('OptionOpaque_returns')
      .asFunction<_ResultOptionStructFfiVoid Function()>(isLeaf: true);

  static final OptionStruct struct = () {
    final result = _OptionOpaque_new_struct();
    return OptionStruct._(result);
  }();

  // ignore: non_constant_identifier_names
  static final _OptionOpaque_new_struct =
    _capi<ffi.NativeFunction<_OptionStructFfi Function()>>('OptionOpaque_new_struct')
      .asFunction<_OptionStructFfi Function()>(isLeaf: true);

  static final OptionStruct structNones = () {
    final result = _OptionOpaque_new_struct_nones();
    return OptionStruct._(result);
  }();

  // ignore: non_constant_identifier_names
  static final _OptionOpaque_new_struct_nones =
    _capi<ffi.NativeFunction<_OptionStructFfi Function()>>('OptionOpaque_new_struct_nones')
      .asFunction<_OptionStructFfi Function()>(isLeaf: true);

  void assertInteger(int i) {
    _OptionOpaque_assert_integer(_underlying, i);
  }

  // ignore: non_constant_identifier_names
  static final _OptionOpaque_assert_integer =
    _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Int32)>>('OptionOpaque_assert_integer')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  static bool optionOpaqueArgument(OptionOpaque? arg) {
    final result = _OptionOpaque_option_opaque_argument(arg == null ? ffi.Pointer.fromAddress(0) : arg._underlying);
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _OptionOpaque_option_opaque_argument =
    _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>>('OptionOpaque_option_opaque_argument')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
