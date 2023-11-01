import 'UnimportedEnum.dart';
import 'dart:ffi' as ffi;

class ImportedStructFfi extends ffi.Struct {
  @ffi.Int32()
  external UnimportedEnumFfi foo;
  @ffi.Uint8()
  external int count;
}

class ImportedStruct {
  final ImportedStructFfi _underlying;
  UnimportedEnum get foo => UnimportedEnumFromFfi(this._underlying.foo);
  void set foo(UnimportedEnum foo) {
    this._underlying.foo = UnimportedEnumAsFfi(foo);
  }

  int get count => this._underlying.count;
  void set count(int count) {
    this._underlying.count = count;
  }

  ImportedStruct._(this._underlying);
}

// These are not methods because we want to keep them package-private, and methods are either private or public
ImportedStruct ImportedStructFromFfi(ImportedStructFfi underlying) =>
    ImportedStruct._(underlying);
ImportedStructFfi ImportedStructAsFfi(ImportedStruct t) => t._underlying;
