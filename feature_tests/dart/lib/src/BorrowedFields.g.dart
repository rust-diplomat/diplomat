// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _BorrowedFieldsFfi extends ffi.Struct {
  external _SliceUtf16 a;
  external _SliceUtf8 b;
  external _SliceUtf8 c;
}

final class BorrowedFields {
  String a;
  String b;
  String c;
  // ignore: unused element
  final core.List<Object> _edgeX;

  BorrowedFields({required this.a, required this.b, required this.c}) : _edgeX = [];

  // ignore: unused_element
  // Internal constructor from FFI.
  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  BorrowedFields._(_BorrowedFieldsFfi underlying, {required core.List<Object> edgeX}) :
    _edgeX = edgeX,
    a = core.String.fromCharCodes(underlying.a._pointer.asTypedList(underlying.a._length)),
    b = Utf8Decoder().convert(underlying.b._pointer.asTypedList(underlying.b._length)),
    c = Utf8Decoder().convert(underlying.c._pointer.asTypedList(underlying.c._length));

  // ignore: unused_element
  _BorrowedFieldsFfi _pointer(ffi.Allocator temp) {
    final pointer = temp<_BorrowedFieldsFfi>();
    final aView = a.utf16View;
    pointer.ref.a._pointer = aView.pointer(temp);
    pointer.ref.a._length = aView.length;
    final bView = b.utf8View;
    pointer.ref.b._pointer = bView.pointer(temp);
    pointer.ref.b._length = bView.length;
    final cView = c.utf8View;
    pointer.ref.c._pointer = cView.pointer(temp);
    pointer.ref.c._length = cView.length;
    return pointer.ref;
  }

  @override
  bool operator ==(Object other) =>
      other is BorrowedFields &&
      other.a == this.a &&
      other.b == this.b &&
      other.c == this.c;

  @override
  int get hashCode => Object.hashAll([
        this.a,
        this.b,
        this.c,
      ]);
}
