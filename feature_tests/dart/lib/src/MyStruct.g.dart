// generated by diplomat-tool
// dart format off

part of 'lib.g.dart';

final class _MyStructFfi extends ffi.Struct {
  @ffi.Uint8()
  external int a;
  @ffi.Bool()
  external bool b;
  @ffi.Uint8()
  external int c;
  @ffi.Uint64()
  external int d;
  @ffi.Int32()
  external int e;
  @ffi.Uint32()
  external Rune f;
  @ffi.Int32()
  external int g;
}

final class MyStruct {
  int a;
  bool b;
  int c;
  int d;
  int e;
  Rune f;
  MyEnum g;

  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  // ignore: unused_element
  MyStruct._fromFfi(_MyStructFfi ffi) :
    a = ffi.a,
    b = ffi.b,
    c = ffi.c,
    d = ffi.d,
    e = ffi.e,
    f = ffi.f,
    g = MyEnum.values.firstWhere((v) => v._ffi == ffi.g);

  // ignore: unused_element
  _MyStructFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_MyStructFfi>();
    struct.a = a;
    struct.b = b;
    struct.c = c;
    struct.d = d;
    struct.e = e;
    struct.f = f;
    struct.g = g._ffi;
    return struct;
  }

  factory MyStruct({int? a, bool? b, int? c, int? d, int? e, Rune? f, MyEnum? g}) {
    final result = _MyStruct_new();
    final dart = MyStruct._fromFfi(result);
    if (a != null) {
      dart.a = a;
    }
    if (b != null) {
      dart.b = b;
    }
    if (c != null) {
      dart.c = c;
    }
    if (d != null) {
      dart.d = d;
    }
    if (e != null) {
      dart.e = e;
    }
    if (f != null) {
      dart.f = f;
    }
    if (g != null) {
      dart.g = g;
    }
    return dart;
  }

  int intoA() {
    final temp = _FinalizedArena();
    final result = _MyStruct_into_a(_toFfi(temp.arena));
    return result;
  }
  ///
  ///
  ///  Throws [MyZst] on failure.
  static void returnsZstResult() {
    final result = _MyStruct_returns_zst_result();
    if (!result.isOk) {
      throw MyZst();
    }
  }
  ///
  ///
  ///  Throws [MyZst] on failure.
  static void failsZstResult() {
    final result = _MyStruct_fails_zst_result();
    if (!result.isOk) {
      throw MyZst();
    }
  }

  @override
  bool operator ==(Object other) =>
      other is MyStruct &&
      other.a == a &&
      other.b == b &&
      other.c == c &&
      other.d == d &&
      other.e == e &&
      other.f == f &&
      other.g == g;

  @override
  int get hashCode => Object.hashAll([
        a,
        b,
        c,
        d,
        e,
        f,
        g,
      ]);
}

@_DiplomatFfiUse('MyStruct_new')
@ffi.Native<_MyStructFfi Function()>(isLeaf: true, symbol: 'MyStruct_new')
// ignore: non_constant_identifier_names
external _MyStructFfi _MyStruct_new();

@_DiplomatFfiUse('MyStruct_into_a')
@ffi.Native<ffi.Uint8 Function(_MyStructFfi)>(isLeaf: true, symbol: 'MyStruct_into_a')
// ignore: non_constant_identifier_names
external int _MyStruct_into_a(_MyStructFfi self);

@_DiplomatFfiUse('MyStruct_returns_zst_result')
@ffi.Native<_ResultVoidMyZstFfi Function()>(isLeaf: true, symbol: 'MyStruct_returns_zst_result')
// ignore: non_constant_identifier_names
external _ResultVoidMyZstFfi _MyStruct_returns_zst_result();

@_DiplomatFfiUse('MyStruct_fails_zst_result')
@ffi.Native<_ResultVoidMyZstFfi Function()>(isLeaf: true, symbol: 'MyStruct_fails_zst_result')
// ignore: non_constant_identifier_names
external _ResultVoidMyZstFfi _MyStruct_fails_zst_result();

// dart format on
