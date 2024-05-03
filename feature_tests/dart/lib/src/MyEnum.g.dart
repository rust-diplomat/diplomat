// generated by diplomat-tool

part of 'lib.g.dart';

enum MyEnum {
  a,

  b,

  c,

  d,

  e,

  f;

  int get _ffi {
    switch (this) {
      case a:
        return -2;
      case b:
        return -1;
      case c:
        return 0;
      case d:
        return 1;
      case e:
        return 2;
      case f:
        return 3;
    }
  }

  int intoValue() {
    final result = _MyEnum_into_value(_ffi);
    return result;
  }

  static MyEnum getA() {
    final result = _MyEnum_get_a();
    return MyEnum.values.firstWhere((v) => v._ffi == result);
  }
}

@meta.ResourceIdentifier('MyEnum_into_value')
@ffi.Native<ffi.Int8 Function(ffi.Int32)>(isLeaf: true, symbol: 'MyEnum_into_value')
// ignore: non_constant_identifier_names
external int _MyEnum_into_value(int self);

@meta.ResourceIdentifier('MyEnum_get_a')
@ffi.Native<ffi.Int32 Function()>(isLeaf: true, symbol: 'MyEnum_get_a')
// ignore: non_constant_identifier_names
external int _MyEnum_get_a();
