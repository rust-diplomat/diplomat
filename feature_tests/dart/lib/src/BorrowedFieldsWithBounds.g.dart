// generated by diplomat-tool

part of 'lib.g.dart';

final class _BorrowedFieldsWithBoundsFfi extends ffi.Struct {
  external _SliceUtf16 fieldA;
  external _SliceUtf8 fieldB;
  external _SliceUtf8 fieldC;
}

final class BorrowedFieldsWithBounds {
  String fieldA;
  String fieldB;
  String fieldC;

  BorrowedFieldsWithBounds({required this.fieldA, required this.fieldB, required this.fieldC});
  // ignore: unused_element
  BorrowedFieldsWithBounds._fromFfi(_BorrowedFieldsWithBoundsFfi ffi, core.List<Object> aEdges, core.List<Object> bEdges, core.List<Object> cEdges) :
    fieldA = core.String.fromCharCodes(ffi.fieldA._data.asTypedList(ffi.fieldA._length)),
    fieldB = Utf8Decoder().convert(ffi.fieldB._data.asTypedList(ffi.fieldB._length)),
    fieldC = Utf8Decoder().convert(ffi.fieldC._data.asTypedList(ffi.fieldC._length));
  // If this struct contains any slices, their lifetime-edge-relevant objects (typically _FinalizedArenas) will only
  // be constructed here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
  // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
  // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
  //
  // This method does not handle lifetime relationships: if `'foo: 'bar`, make sure fooAppendArray contains everything barAppendArray does.
  // ignore: unused_element
  _BorrowedFieldsWithBoundsFfi _toFfi(ffi.Allocator temp, {core.List<core.List<Object>>? aAppendArray, core.List<core.List<Object>>? bAppendArray, core.List<core.List<Object>>? cAppendArray}) {
    final struct = ffi.Struct.create<_BorrowedFieldsWithBoundsFfi>();
    final fieldAView = fieldA.utf16View;
    struct.fieldA._length = fieldAView.length;
    final fieldAArena = (aAppendArray != null && !aAppendArray.isEmpty) ? _FinalizedArena.withLifetime(aAppendArray).arena : temp;
    struct.fieldA._data = fieldAView.allocIn(fieldAArena);
    final fieldBView = fieldB.utf8View;
    struct.fieldB._length = fieldBView.length;
    final fieldBArena = (bAppendArray != null && !bAppendArray.isEmpty) ? _FinalizedArena.withLifetime(bAppendArray).arena : temp;
    struct.fieldB._data = fieldBView.allocIn(fieldBArena);
    final fieldCView = fieldC.utf8View;
    struct.fieldC._length = fieldCView.length;
    final fieldCArena = (cAppendArray != null && !cAppendArray.isEmpty) ? _FinalizedArena.withLifetime(cAppendArray).arena : temp;
    struct.fieldC._data = fieldCView.allocIn(fieldCArena);
    return struct;
  }

  @override
  bool operator ==(Object other) =>
      other is BorrowedFieldsWithBounds &&
      other.fieldA == this.fieldA &&
      other.fieldB == this.fieldB &&
      other.fieldC == this.fieldC;

  @override
  int get hashCode => Object.hashAll([
        this.fieldA,
        this.fieldB,
        this.fieldC,
      ]);

  // Append all fields corresponding to lifetime `'a` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'a`,
  // assuming that there are no `'other: a`. bounds. In case of such bounds,
  // the caller should take care to also call _fields_for_lifetime_other()
  // ignore: unused_element
  core.List<Object> _fields_for_lifetime_a() {
    return [fieldA];
  }

  // Append all fields corresponding to lifetime `'b` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'b`,
  // assuming that there are no `'other: b`. bounds. In case of such bounds,
  // the caller should take care to also call _fields_for_lifetime_other()
  // ignore: unused_element
  core.List<Object> _fields_for_lifetime_b() {
    return [fieldB];
  }

  // Append all fields corresponding to lifetime `'c` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'c`,
  // assuming that there are no `'other: c`. bounds. In case of such bounds,
  // the caller should take care to also call _fields_for_lifetime_other()
  // ignore: unused_element
  core.List<Object> _fields_for_lifetime_c() {
    return [fieldC];
  }
}
