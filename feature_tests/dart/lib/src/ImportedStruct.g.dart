// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _ImportedStructFfi extends ffi.Struct {
  @ffi.Int32()
  external int foo;
  @ffi.Uint8()
  external int count;
}

final class ImportedStruct {
  UnimportedEnum foo;
  int count;

  ImportedStruct({UnimportedEnum this.foo = UnimportedEnum.a, int this.count = 0});

  // ignore: unused_element
  ImportedStruct._(_ImportedStructFfi underlying) :
    foo = UnimportedEnum.values[underlying.foo],
    count = underlying.count;

  // ignore: unused_element
  _ImportedStructFfi _pointer(ffi.Allocator temp) {
    final pointer = temp<_ImportedStructFfi>();
    pointer.ref.foo = foo.index;
    pointer.ref.count = count;
    return pointer.ref;
  }

  @override
  bool operator ==(Object other) =>
      other is ImportedStruct &&
      other.foo == this.foo &&
      other.count == this.count;

  @override
  int get hashCode => Object.hashAll([
        this.foo,
        this.count,
      ]);
}
