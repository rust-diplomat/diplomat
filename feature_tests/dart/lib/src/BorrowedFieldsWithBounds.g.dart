// generated by diplomat-tool
// dart format off

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
    fieldA = ffi.fieldA._toDart(aEdges),
    fieldB = ffi.fieldB._toDart(bEdges),
    fieldC = ffi.fieldC._toDart(cEdges);

  // If this struct contains any slices, their lifetime-edge-relevant objects (typically _FinalizedArenas) will only
  // be constructed here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
  // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
  // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
  //
  // This method does not handle lifetime relationships: if `'foo: 'bar`, make sure fooAppendArray contains everything barAppendArray does.
  // ignore: unused_element
  _BorrowedFieldsWithBoundsFfi _toFfi(ffi.Allocator temp, {core.List<core.List<Object>> aAppendArray = const [], core.List<core.List<Object>> bAppendArray = const [], core.List<core.List<Object>> cAppendArray = const []}) {
    final struct = ffi.Struct.create<_BorrowedFieldsWithBoundsFfi>();
    struct.fieldA = fieldA._utf16AllocIn(aAppendArray.isNotEmpty ? _FinalizedArena.withLifetime(aAppendArray).arena : temp);
    struct.fieldB = fieldB._utf8AllocIn(bAppendArray.isNotEmpty ? _FinalizedArena.withLifetime(bAppendArray).arena : temp);
    struct.fieldC = fieldC._utf8AllocIn(cAppendArray.isNotEmpty ? _FinalizedArena.withLifetime(cAppendArray).arena : temp);
    return struct;
  }

  static BorrowedFieldsWithBounds fromFooAndStrings(Foo foo, String dstr16X, String utf8StrZ) {
    final dstr16XArena = _FinalizedArena();
    final utf8StrZArena = _FinalizedArena();
    // This lifetime edge depends on lifetimes: 'x, 'y, 'z
    core.List<Object> xEdges = [foo, dstr16XArena, utf8StrZArena];
    // This lifetime edge depends on lifetimes: 'y, 'z
    core.List<Object> yEdges = [foo, utf8StrZArena];
    // This lifetime edge depends on lifetimes: 'z
    core.List<Object> zEdges = [utf8StrZArena];
    final result = _BorrowedFieldsWithBounds_from_foo_and_strings(foo._ffi, dstr16X._utf16AllocIn(dstr16XArena.arena), utf8StrZ._utf8AllocIn(utf8StrZArena.arena));
    return BorrowedFieldsWithBounds._fromFfi(result, xEdges, yEdges, zEdges);
  }

  @override
  bool operator ==(Object other) =>
      other is BorrowedFieldsWithBounds &&
      other.fieldA == fieldA &&
      other.fieldB == fieldB &&
      other.fieldC == fieldC;

  @override
  int get hashCode => Object.hashAll([
        fieldA,
        fieldB,
        fieldC,
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

@_DiplomatFfiUse('BorrowedFieldsWithBounds_from_foo_and_strings')
@ffi.Native<_BorrowedFieldsWithBoundsFfi Function(ffi.Pointer<ffi.Opaque>, _SliceUtf16, _SliceUtf8)>(isLeaf: true, symbol: 'BorrowedFieldsWithBounds_from_foo_and_strings')
// ignore: non_constant_identifier_names
external _BorrowedFieldsWithBoundsFfi _BorrowedFieldsWithBounds_from_foo_and_strings(ffi.Pointer<ffi.Opaque> foo, _SliceUtf16 dstr16X, _SliceUtf8 utf8StrZ);

// dart format on
