// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

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
  int _a;
  bool _b;
  int _c;
  int _d;
  int _e;
  Rune _f;
  MyEnum _g;

  MyStruct._(this._a,this._b,this._c,this._d,this._e,this._f,this._g,);


  factory MyStruct._fromFfi(_MyStructFfi ffi){

    var _underlying = ffi;
    var _a = _underlying.a;
    var _b = _underlying.b;
    var _c = _underlying.c;
    var _d = _underlying.d;
    var _e = _underlying.e;
    var _f = _underlying.f;
    var _g = MyEnum.values.firstWhere((v) => v._underlying == _underlying.g);
    return MyStruct._(_a, _b, _c, _d, _e, _f, _g, );
  }

  _MyStructFfi _toFfi() {
    final pointer = ffi2.calloc<_MyStructFfi>();
    var _underlying = pointer.ref;
    _underlying.a = a;;
    _underlying.b = b;;
    _underlying.c = c;;
    _underlying.d = d;;
    _underlying.e = e;;
    _underlying.f = f;;
    _underlying.g = g._underlying;;

    _callocFree.attach(_underlying, pointer.cast());
    return _underlying;
  }

  int get a => this._a;
  set a(int a) {
    _a = a;
  }

  bool get b => this._b;
  set b(bool b) {
    _b = b;
  }

  int get c => this._c;
  set c(int c) {
    _c = c;
  }

  int get d => this._d;
  set d(int d) {
    _d = d;
  }

  int get e => this._e;
  set e(int e) {
    _e = e;
  }

  Rune get f => this._f;
  set f(Rune f) {
    _f = f;
  }

  MyEnum get g => this._g;
  set g(MyEnum g) {
    _g = g;
  }

  factory MyStruct() {
    final result = _MyStruct_new();
    return MyStruct._fromFfi(result);
  }

  int intoA() {
    final result = _MyStruct_into_a(_underlying);
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
