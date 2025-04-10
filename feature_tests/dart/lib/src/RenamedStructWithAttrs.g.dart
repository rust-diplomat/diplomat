// generated by diplomat-tool
// dart format off

part of 'lib.g.dart';

final class _RenamedStructWithAttrsFfi extends ffi.Struct {
  @ffi.Bool()
  external bool a;
  @ffi.Uint32()
  external int b;
}

final class RenamedStructWithAttrs {
  bool a;
  int b;

  RenamedStructWithAttrs({required this.a, required this.b});

  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  // ignore: unused_element
  RenamedStructWithAttrs._fromFfi(_RenamedStructWithAttrsFfi ffi) :
    a = ffi.a,
    b = ffi.b;

  // ignore: unused_element
  _RenamedStructWithAttrsFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_RenamedStructWithAttrsFfi>();
    struct.a = a;
    struct.b = b;
    return struct;
  }

  int get c {
    final temp = _FinalizedArena();
    final result = _namespace_StructWithAttrs_c(_toFfi(temp.arena));
    return result;
  }


  @override
  bool operator ==(Object other) =>
      other is RenamedStructWithAttrs &&
      other.a == a &&
      other.b == b;

  @override
  int get hashCode => Object.hashAll([
        a,
        b,
      ]);
}

@_DiplomatFfiUse('namespace_StructWithAttrs_c')
@ffi.Native<ffi.Uint32 Function(_RenamedStructWithAttrsFfi)>(isLeaf: true, symbol: 'namespace_StructWithAttrs_c')
// ignore: non_constant_identifier_names
external int _namespace_StructWithAttrs_c(_RenamedStructWithAttrsFfi self);

// dart format on
