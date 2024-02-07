// generated by diplomat-tool

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

  // Internal constructor from FFI.
  // ignore: unused_element
  MyStruct._(_MyStructFfi underlying) :
    a = underlying.a,
    b = underlying.b,
    c = underlying.c,
    d = underlying.d,
    e = underlying.e,
    f = underlying.f,
    g = MyEnum.values.firstWhere((v) => v._underlying == underlying.g);

  // ignore: unused_element
  _MyStructFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_MyStructFfi>();
    struct.a = a;
    struct.b = b;
    struct.c = c;
    struct.d = d;
    struct.e = e;
    struct.f = f;
    struct.g = g._underlying;
    return struct;
  }

  factory MyStruct({int? a, bool? b, int? c, int? d, int? e, Rune? f, MyEnum? g}) {
    final result = _MyStruct_new();
    final dart = MyStruct._(result);
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
    final temp = ffi2.Arena();
    final result = _MyStruct_into_a(_toFfi(temp));
    temp.releaseAll();
    return result;
  }

  @override
  bool operator ==(Object other) =>
      other is MyStruct &&
      other.a == this.a &&
      other.b == this.b &&
      other.c == this.c &&
      other.d == this.d &&
      other.e == this.e &&
      other.f == this.f &&
      other.g == this.g;

  @override
  int get hashCode => Object.hashAll([
        this.a,
        this.b,
        this.c,
        this.d,
        this.e,
        this.f,
        this.g,
      ]);
}

@ffi.Native<_MyStructFfi Function()>(isLeaf: true, symbol: 'MyStruct_new')
// ignore: non_constant_identifier_names
external _MyStructFfi _MyStruct_new();

@ffi.Native<ffi.Uint8 Function(_MyStructFfi)>(isLeaf: true, symbol: 'MyStruct_into_a')
// ignore: non_constant_identifier_names
external int _MyStruct_into_a(_MyStructFfi self);
