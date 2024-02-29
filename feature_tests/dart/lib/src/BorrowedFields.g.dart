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

  BorrowedFields({required this.a, required this.b, required this.c});

  // ignore: unused_element
  // Internal constructor from FFI.
  BorrowedFields._(_BorrowedFieldsFfi underlying, core.List<Object> edge_a) :
    a = core.String.fromCharCodes(underlying.a._pointer.asTypedList(underlying.a._length)),
    b = Utf8Decoder().convert(underlying.b._pointer.asTypedList(underlying.b._length)),
    c = Utf8Decoder().convert(underlying.c._pointer.asTypedList(underlying.c._length));

  // ignore: unused_element
  // If this struct contains any slices, their lifetime-edge-relevant objects (typically _FinalizedArenas) will only
  // be constructed here, and can be appended to any relevant lifetime arrays here. append_array_for_<lifetime> accepts a list
  // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
  // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
  _BorrowedFieldsFfi _pointer(ffi.Allocator temp, {core.List<core.List<Object>>? append_array_for_a}) {
    final pointer = temp<_BorrowedFieldsFfi>();
    final aView = a.utf16View;
    pointer.ref.a._length = aView.length;
    var aArena = temp;
    if (append_array_for_a != null && !append_array_for_a.isEmpty) {
      final aFinalizedArena = _FinalizedArena();
      aArena = aFinalizedArena.arena;
      for(final edge in append_array_for_a) {
        edge.add(aFinalizedArena);
      }
    }
    pointer.ref.a._pointer = aView.pointer(aArena);
    final bView = b.utf8View;
    pointer.ref.b._length = bView.length;
    var bArena = temp;
    if (append_array_for_a != null && !append_array_for_a.isEmpty) {
      final bFinalizedArena = _FinalizedArena();
      bArena = bFinalizedArena.arena;
      for(final edge in append_array_for_a) {
        edge.add(bFinalizedArena);
      }
    }
    pointer.ref.b._pointer = bView.pointer(bArena);
    final cView = c.utf8View;
    pointer.ref.c._length = cView.length;
    var cArena = temp;
    if (append_array_for_a != null && !append_array_for_a.isEmpty) {
      final cFinalizedArena = _FinalizedArena();
      cArena = cFinalizedArena.arena;
      for(final edge in append_array_for_a) {
        edge.add(cFinalizedArena);
      }
    }
    pointer.ref.c._pointer = cView.pointer(cArena);
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

  // ignore: unused element
  // Append all fields corresponding to lifetime `'a` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'a`,
  // assuming that there are no `'other: a`. bounds. In case of such bounds,
  // the caller should take care to also call _fields_for_lifetime_other()
  core.List<Object> _fields_for_lifetime_a() {
    return [a, b, c];
  }
}
