// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class Foo implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;
  final core.List<Object> _edge_a;

  // Internal constructor from FFI.
  // isOwned is whether this is owned (has finalizer) or not
  // This also takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  Foo._(this._underlying, bool isOwned, this._edge_self, this._edge_a) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_Foo_destroy));

  factory Foo(String x) {
    final xView = x.utf8View;
    final xArena = _FinalizedArena();
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> edge_a = [xArena];
    final result = _Foo_new(xView.pointer(xArena.arena), xView.length);
    return Foo._(result, true, [], edge_a);
  }

  Bar get getBar {
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> edge_a = [this];
    // This lifetime edge depends on lifetimes: 'a, 'b
    core.List<Object> edge_b = [this];
    final result = _Foo_get_bar(_underlying);
    return Bar._(result, true, [], edge_b, edge_a);
  }

  factory Foo.static_(String x) {
    final temp = ffi2.Arena();
    final xView = x.utf8View;
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> edge_a = [];
    final result = _Foo_new_static(xView.pointer(temp), xView.length);
    temp.releaseAll();
    return Foo._(result, true, [], edge_a);
  }

  BorrowedFieldsReturning get asReturning {
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> edge_a = [this];
    final result = _Foo_as_returning(_underlying);
    return BorrowedFieldsReturning._(result, []);
  }

  factory Foo.extractFromFields(BorrowedFields fields) {
    final temp = ffi2.Arena();
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> edge_a = [...fields._fields_for_lifetime_a()];
    final result = _Foo_extract_from_fields(fields._pointer(temp));
    temp.releaseAll();
    return Foo._(result, true, [], edge_a);
  }

  /// Test that the extraction logic correctly pins the right fields
  factory Foo.extractFromBounds(BorrowedFieldsWithBounds bounds, String anotherString) {
    final temp = ffi2.Arena();
    final anotherStringView = anotherString.utf8View;
    final anotherStringArena = _FinalizedArena();
    // This lifetime edge depends on lifetimes: 'a, 'y
    core.List<Object> edge_a = [...bounds._fields_for_lifetime_b(), anotherStringArena];
    final result = _Foo_extract_from_bounds(bounds._pointer(temp), anotherStringView.pointer(anotherStringArena.arena), anotherStringView.length);
    temp.releaseAll();
    return Foo._(result, true, [], edge_a);
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

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(_BorrowedFieldsWithBoundsFfi, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'Foo_extract_from_bounds')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _Foo_extract_from_bounds(_BorrowedFieldsWithBoundsFfi bounds, ffi.Pointer<ffi.Uint8> anotherStringData, int anotherStringLength);
