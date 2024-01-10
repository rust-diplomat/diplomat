// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class AttrOpaque1 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  AttrOpaque1._(this._underlying, bool isOwned) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_AttrOpaque1_destroy));

  void method() {
    _AttrOpaque1_method(_underlying);
  }

  void methodDisabledcpp() {
    _AttrOpaque1_method_disabledcpp(_underlying);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'AttrOpaque1_destroy')
// ignore: non_constant_identifier_names
external void _AttrOpaque1_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'AttrOpaque1_method')
// ignore: non_constant_identifier_names
external void _AttrOpaque1_method(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'AttrOpaque1_method_disabledcpp')
// ignore: non_constant_identifier_names
external void _AttrOpaque1_method_disabledcpp(ffi.Pointer<ffi.Opaque> self);
