// generated by diplomat-tool
// dart format off

part of 'lib.g.dart';

enum DefaultEnum {

  a,

  b;

  static DefaultEnum new_() {
    final result = _DefaultEnum_new();
    return DefaultEnum.values[result];
  }

}

@_DiplomatFfiUse('DefaultEnum_new')
@ffi.Native<ffi.Int32 Function()>(isLeaf: true, symbol: 'DefaultEnum_new')
// ignore: non_constant_identifier_names
external int _DefaultEnum_new();

// dart format on
