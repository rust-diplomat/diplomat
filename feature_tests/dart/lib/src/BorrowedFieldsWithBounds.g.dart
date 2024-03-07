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
  _BorrowedFieldsWithBoundsFfi _toFfi(ffi.Allocator temp, {core.List<core.List<Object>> aAppendArray = const [], core.List<core.List<Object>> bAppendArray = const [], core.List<core.List<Object>> cAppendArray = const []}) {
    final struct = ffi.Struct.create<_BorrowedFieldsWithBoundsFfi>();
    final fieldAView = fieldA.utf16View;
    struct.fieldA._length = fieldAView.length;
    struct.fieldA._data = fieldAView.allocIn(aAppendArray.isNotEmpty ? _FinalizedArena.withLifetime(aAppendArray).arena : temp);
    final fieldBView = fieldB.utf8View;
    struct.fieldB._length = fieldBView.length;
    struct.fieldB._data = fieldBView.allocIn(bAppendArray.isNotEmpty ? _FinalizedArena.withLifetime(bAppendArray).arena : temp);
    final fieldCView = fieldC.utf8View;
    struct.fieldC._length = fieldCView.length;
    struct.fieldC._data = fieldCView.allocIn(cAppendArray.isNotEmpty ? _FinalizedArena.withLifetime(cAppendArray).arena : temp);
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

  // Return all fields corresponding to lifetime `'a` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'a`,
  // assuming that there are no `'other: a`. bounds. In case of such bounds,
  // the caller should take care to also call _fieldsForLifetimeOther
  // ignore: unused_element
  core.List<Object> get _fieldsForLifetimeA => [fieldA];

  // Return all fields corresponding to lifetime `'b` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'b`,
  // assuming that there are no `'other: b`. bounds. In case of such bounds,
  // the caller should take care to also call _fieldsForLifetimeOther
  // ignore: unused_element
  core.List<Object> get _fieldsForLifetimeB => [fieldB];

  // Return all fields corresponding to lifetime `'c` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'c`,
  // assuming that there are no `'other: c`. bounds. In case of such bounds,
  // the caller should take care to also call _fieldsForLifetimeOther
  // ignore: unused_element
  core.List<Object> get _fieldsForLifetimeC => [fieldC];
}
