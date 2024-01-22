// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class Bar implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;
  final core.List<Object> _edge_b;
  final core.List<Object> _edge_a;

  Bar._(this._underlying, bool isOwned, this._edge_self, this._edge_b, this._edge_a) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_Bar_destroy));

  Foo get foo {
    final result = _Bar_foo(_underlying);
    return Foo._(result, false, [], []);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'Bar_destroy')
// ignore: non_constant_identifier_names
external void _Bar_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'Bar_foo')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Bar_foo(ffi.Pointer<ffi.Opaque> self);
