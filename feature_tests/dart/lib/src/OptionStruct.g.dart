// generated by diplomat-tool

part of 'lib.g.dart';

final class _OptionStructFfi extends ffi.Struct {
  external ffi.Pointer<ffi.Opaque> a;
  external ffi.Pointer<ffi.Opaque> b;
  @ffi.Uint32()
  external int c;
  external ffi.Pointer<ffi.Opaque> d;
}

final class OptionStruct {
  final OptionOpaque? a;
  final OptionOpaqueChar? b;
  final int c;
  final OptionOpaque? d;
  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  // ignore: unused_element
  OptionStruct._fromFfi(_OptionStructFfi ffi) :
    a = ffi.a.address == 0 ? null : OptionOpaque._fromFfi(ffi.a, true, []),
    b = ffi.b.address == 0 ? null : OptionOpaqueChar._fromFfi(ffi.b, true, []),
    c = ffi.c,
    d = ffi.d.address == 0 ? null : OptionOpaque._fromFfi(ffi.d, true, []);
  // ignore: unused_element
  _OptionStructFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_OptionStructFfi>();
    struct.a = a?._ffi ?? ffi.Pointer.fromAddress(0);
    struct.b = b?._ffi ?? ffi.Pointer.fromAddress(0);
    struct.c = c;
    struct.d = d?._ffi ?? ffi.Pointer.fromAddress(0);
    return struct;
  }

  @override
  bool operator ==(Object other) =>
      other is OptionStruct &&
      other.a == this.a &&
      other.b == this.b &&
      other.c == this.c &&
      other.d == this.d;

  @override
  int get hashCode => Object.hashAll([
        this.a,
        this.b,
        this.c,
        this.d,
      ]);
}
