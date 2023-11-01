import 'MyEnum.dart';
import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

class MyStructFfi extends ffi.Struct {
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
  external int f;
  @ffi.Int32()
  external MyEnumFfi g;
}

class MyStruct {
  final MyStructFfi _underlying;
  int get a => this._underlying.a;
  void set a(int a) {
    this._underlying.a = a;
  }

  bool get b => this._underlying.b;
  void set b(bool b) {
    this._underlying.b = b;
  }

  int get c => this._underlying.c;
  void set c(int c) {
    this._underlying.c = c;
  }

  int get d => this._underlying.d;
  void set d(int d) {
    this._underlying.d = d;
  }

  int get e => this._underlying.e;
  void set e(int e) {
    this._underlying.e = e;
  }

  int get f => this._underlying.f;
  void set f(int f) {
    this._underlying.f = f;
  }

  MyEnum get g => MyEnumFromFfi(this._underlying.g);
  void set g(MyEnum g) {
    this._underlying.g = MyEnumAsFfi(g);
  }

  factory MyStruct.new() {
    final result = _newFfi();
    return MyStructFromFfi(result);
  }
  static late final _newFfi =
      capi<ffi.NativeFunction<MyStructFfi Function()>>('MyStruct_new')
          .asFunction<MyStructFfi Function()>();
  int intoA() {
    final result = _intoAFfi(this._underlying);
    return result;
  }

  static late final _intoAFfi =
      capi<ffi.NativeFunction<ffi.Uint8 Function(MyStructFfi)>>(
              'MyStruct_into_a')
          .asFunction<int Function(MyStructFfi)>();
  MyStruct._(this._underlying);
}

// These are not methods because we want to keep them package-private, and methods are either private or public
MyStruct MyStructFromFfi(MyStructFfi underlying) => MyStruct._(underlying);
MyStructFfi MyStructAsFfi(MyStruct t) => t._underlying;
