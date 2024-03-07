// generated by diplomat-tool

part of 'lib.g.dart';

final class _NestedBorrowedFieldsFfi extends ffi.Struct {
  external _BorrowedFieldsFfi fields;
  external _BorrowedFieldsWithBoundsFfi bounds;
  external _BorrowedFieldsWithBoundsFfi bounds2;
}

final class NestedBorrowedFields {
  BorrowedFields fields;
  BorrowedFieldsWithBounds bounds;
  BorrowedFieldsWithBounds bounds2;

  NestedBorrowedFields({required this.fields, required this.bounds, required this.bounds2});

  // ignore: unused_element
  NestedBorrowedFields._fromFfi(_NestedBorrowedFieldsFfi ffi, core.List<Object> xEdges, core.List<Object> yEdges, core.List<Object> zEdges) :
    fields = BorrowedFields._fromFfi(ffi.fields, xEdges),
    bounds = BorrowedFieldsWithBounds._fromFfi(ffi.bounds, xEdges, yEdges, yEdges),
    bounds2 = BorrowedFieldsWithBounds._fromFfi(ffi.bounds2, zEdges, zEdges, zEdges);

  // If this struct contains any slices, their lifetime-edge-relevant objects (typically _FinalizedArenas) will only
  // be constructed here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
  // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
  // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
  //
  // This method does not handle lifetime relationships: if `'foo: 'bar`, make sure fooAppendArray contains everything barAppendArray does.
  // ignore: unused_element
  _NestedBorrowedFieldsFfi _toFfi(ffi.Allocator temp, {core.List<core.List<Object>> xAppendArray = const [], core.List<core.List<Object>> yAppendArray = const [], core.List<core.List<Object>> zAppendArray = const []}) {
    final struct = ffi.Struct.create<_NestedBorrowedFieldsFfi>();
    struct.fields = fields._toFfi(temp, aAppendArray: [...xAppendArray]);
    struct.bounds = bounds._toFfi(temp, aAppendArray: [...xAppendArray], bAppendArray: [...yAppendArray], cAppendArray: [...yAppendArray]);
    struct.bounds2 = bounds2._toFfi(temp, aAppendArray: [...zAppendArray], bAppendArray: [...zAppendArray], cAppendArray: [...zAppendArray]);
    return struct;
  }

  @override
  bool operator ==(Object other) =>
      other is NestedBorrowedFields &&
      other.fields == this.fields &&
      other.bounds == this.bounds &&
      other.bounds2 == this.bounds2;

  @override
  int get hashCode => Object.hashAll([
        this.fields,
        this.bounds,
        this.bounds2,
      ]);

  // Append all fields corresponding to lifetime `'x` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'x`,
  // assuming that there are no `'other: x`. bounds. In case of such bounds,
  // the caller should take care to also call _fields_for_lifetime_other()
  // ignore: unused_element
  core.List<Object> _fields_for_lifetime_x() {
    return [...fields._fields_for_lifetime_a(), ...bounds._fields_for_lifetime_a()];
  }

  // Append all fields corresponding to lifetime `'y` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'y`,
  // assuming that there are no `'other: y`. bounds. In case of such bounds,
  // the caller should take care to also call _fields_for_lifetime_other()
  // ignore: unused_element
  core.List<Object> _fields_for_lifetime_y() {
    return [...bounds._fields_for_lifetime_b(), ...bounds._fields_for_lifetime_c()];
  }

  // Append all fields corresponding to lifetime `'z` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'z`,
  // assuming that there are no `'other: z`. bounds. In case of such bounds,
  // the caller should take care to also call _fields_for_lifetime_other()
  // ignore: unused_element
  core.List<Object> _fields_for_lifetime_z() {
    return [...bounds2._fields_for_lifetime_a(), ...bounds2._fields_for_lifetime_b(), ...bounds2._fields_for_lifetime_c()];
  }
}
