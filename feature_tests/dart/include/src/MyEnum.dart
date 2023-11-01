import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef MyEnumFfi = int;

enum MyEnum {
  A._(-2),
  B._(-1),
  C._(0),
  D._(1),
  E._(2),
  F._(3);

  const MyEnum._(this._id);

  final int _id;

  int intoValue() {
    final result = _intoValueFfi(this._id);
    return result;
  }

  static late final _intoValueFfi =
      capi<ffi.NativeFunction<ffi.Int8 Function(ffi.Uint32)>>(
              'MyEnum_into_value')
          .asFunction<int Function(int)>();
}

// These are not methods because we want to keep them package-private, and methods are either private or public
MyEnum MyEnumFromFfi(int id) =>
    MyEnum.values.firstWhere((value) => value._id == id);
MyEnumFfi MyEnumAsFfi(MyEnum t) => t._id;
