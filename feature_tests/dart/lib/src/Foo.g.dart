// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class Foo implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;
  final core.List<Object> _edge_a;

  Foo._(this._underlying, bool isOwned, this._edge_self, this._edge_a) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_Foo_destroy));

  factory Foo(String x) {
    final temp = ffi2.Arena();
    final xView = x.utf8View;
    final result = _Foo_new(xView.pointer(temp), xView.length);
    temp.releaseAll();
    return Foo._(result, true, [], []);
  }

  Bar get getBar {
    final result = _Foo_get_bar(_underlying);
    return Bar._(result, true, [], [], []);
  }

  factory Foo.static_(String x) {
    final temp = ffi2.Arena();
    final xView = x.utf8View;
    final result = _Foo_new_static(xView.pointer(temp), xView.length);
    temp.releaseAll();
    return Foo._(result, true, [], []);
  }

  BorrowedFieldsReturning get asReturning {
    final result = _Foo_as_returning(_underlying);
    return BorrowedFieldsReturning._(result, []);
  }

  factory Foo.extractFromFields(BorrowedFields fields) {
    final temp = ffi2.Arena();
    final result = _Foo_extract_from_fields(fields._pointer(temp));
    temp.releaseAll();
    return Foo._(result, true, [], []);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'Foo_destroy')
// ignore: non_constant_identifier_names
external void _Foo_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'Foo_new')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Foo_new(ffi.Pointer<ffi.Uint8> xData, int xLength);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'Foo_get_bar')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Foo_get_bar(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'Foo_new_static')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Foo_new_static(ffi.Pointer<ffi.Uint8> xData, int xLength);

@ffi.Native<_BorrowedFieldsReturningFfi Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'Foo_as_returning')
// ignore: non_constant_identifier_names
external _BorrowedFieldsReturningFfi _Foo_as_returning(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(_BorrowedFieldsFfi)>(isLeaf: true, symbol: 'Foo_extract_from_fields')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Foo_extract_from_fields(_BorrowedFieldsFfi fields);
