// generated by diplomat-tool

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
  BorrowedFields._fromFfi(_BorrowedFieldsFfi ffi, core.List<Object> aEdges) :
    a = ffi.a._toDart(aEdges),
    b = ffi.b._toDart(aEdges),
    c = ffi.c._toDart(aEdges);

  // If this struct contains any slices, their lifetime-edge-relevant objects (typically _FinalizedArenas) will only
  // be constructed here, and can be appended to any relevant lifetime arrays here. <lifetime>AppendArray accepts a list
  // of arrays for each lifetime to do so. It accepts multiple lists per lifetime in case the caller needs to tie a lifetime to multiple
  // output arrays. Null is equivalent to an empty list: this lifetime is not being borrowed from.
  // ignore: unused_element
  _BorrowedFieldsFfi _toFfi(ffi.Allocator temp, {core.List<core.List<Object>> aAppendArray = const []}) {
    final struct = ffi.Struct.create<_BorrowedFieldsFfi>();
    struct.a = a._utf16AllocIn(aAppendArray.isNotEmpty ? _FinalizedArena.withLifetime(aAppendArray).arena : temp);
    struct.b = b._utf8AllocIn(aAppendArray.isNotEmpty ? _FinalizedArena.withLifetime(aAppendArray).arena : temp);
    struct.c = c._utf8AllocIn(aAppendArray.isNotEmpty ? _FinalizedArena.withLifetime(aAppendArray).arena : temp);
    return struct;
  }

  static BorrowedFields fromBarAndStrings(Bar bar, String dstr16, String utf8Str) {
    final dstr16Arena = _FinalizedArena();
    final utf8StrArena = _FinalizedArena();
    // This lifetime edge depends on lifetimes: 'x
    core.List<Object> xEdges = [bar, dstr16Arena, utf8StrArena];
    final result = _BorrowedFields_from_bar_and_strings(bar._ffi, dstr16._utf16AllocIn(dstr16Arena.arena), utf8Str._utf8AllocIn(utf8StrArena.arena));
    return BorrowedFields._fromFfi(result, xEdges);
  }

  @override
  bool operator ==(Object other) =>
      other is BorrowedFields &&
      other.a == a &&
      other.b == b &&
      other.c == c;

  @override
  int get hashCode => Object.hashAll([
        a,
        b,
        c,
      ]);

  // Return all fields corresponding to lifetime `'a` 
  // without handling lifetime dependencies (this is the job of the caller)
  // This is all fields that may be borrowed from if borrowing `'a`,
  // assuming that there are no `'other: a`. bounds. In case of such bounds,
  // the caller should take care to also call _fieldsForLifetimeOther
  // ignore: unused_element
  core.List<Object> get _fieldsForLifetimeA => [a, b, c];
}

@_DiplomatFfiUse('BorrowedFields_from_bar_and_strings')
@ffi.Native<_BorrowedFieldsFfi Function(ffi.Pointer<ffi.Opaque>, _SliceUtf16, _SliceUtf8)>(isLeaf: true, symbol: 'BorrowedFields_from_bar_and_strings')
// ignore: non_constant_identifier_names
external _BorrowedFieldsFfi _BorrowedFields_from_bar_and_strings(ffi.Pointer<ffi.Opaque> bar, _SliceUtf16 dstr16, _SliceUtf8 utf8Str);
