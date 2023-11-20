// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:typed_data';
import 'package:ffi/ffi.dart' as ffi2;
part 'AttrEnum.g.dart';
part 'AttrOpaque1.g.dart';
part 'AttrOpaque2.g.dart';
part 'Bar.g.dart';
part 'BorrowedFields.g.dart';
part 'BorrowedFieldsReturning.g.dart';
part 'ContiguousEnum.g.dart';
part 'ErrorEnum.g.dart';
part 'ErrorStruct.g.dart';
part 'Float64Vec.g.dart';
part 'Foo.g.dart';
part 'ImportedStruct.g.dart';
part 'MyEnum.g.dart';
part 'MyString.g.dart';
part 'MyStruct.g.dart';
part 'One.g.dart';
part 'Opaque.g.dart';
part 'OptionOpaque.g.dart';
part 'OptionOpaqueChar.g.dart';
part 'OptionStruct.g.dart';
part 'RefList.g.dart';
part 'RefListParameter.g.dart';
part 'ResultOpaque.g.dart';
part 'Two.g.dart';
part 'UnimportedEnum.g.dart';

late final ffi.Pointer<T> Function<T extends ffi.NativeType>(String) _capi;
void init(String path) => _capi = ffi.DynamicLibrary.open(path).lookup;

final _callocFree = Finalizer(ffi2.calloc.free);

final class _ResultInt32OpaqueUnion extends ffi.Union {
  @ffi.Int32()
  external int ok;

  external ffi.Pointer<ffi.Opaque> err;
}

final class _ResultInt32Opaque extends ffi.Struct {
  external _ResultInt32OpaqueUnion union;

  @ffi.Bool()
  external bool isOk;
}

final class _ResultInt32VoidUnion extends ffi.Union {
  @ffi.Int32()
  external int ok;
}

final class _ResultInt32Void extends ffi.Struct {
  external _ResultInt32VoidUnion union;

  @ffi.Bool()
  external bool isOk;
}

final class _ResultOpaqueErrorStructFfiUnion extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> ok;

  external _ErrorStructFfi err;
}

final class _ResultOpaqueErrorStructFfi extends ffi.Struct {
  external _ResultOpaqueErrorStructFfiUnion union;

  @ffi.Bool()
  external bool isOk;
}

final class _ResultOpaqueInt32Union extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> ok;

  @ffi.Int32()
  external int err;
}

final class _ResultOpaqueInt32 extends ffi.Struct {
  external _ResultOpaqueInt32Union union;

  @ffi.Bool()
  external bool isOk;
}

final class _ResultOpaqueVoidUnion extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> ok;
}

final class _ResultOpaqueVoid extends ffi.Struct {
  external _ResultOpaqueVoidUnion union;

  @ffi.Bool()
  external bool isOk;
}

final class _ResultVoidOpaqueUnion extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> err;
}

final class _ResultVoidOpaque extends ffi.Struct {
  external _ResultVoidOpaqueUnion union;

  @ffi.Bool()
  external bool isOk;
}

final class _SliceFfi2Utf8 extends ffi.Struct {
  external ffi.Pointer<ffi2.Utf8> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfi2Utf8 _fromDart(String value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfi2Utf8>();
    final slice = pointer.ref;
    final units = Utf8Encoder().convert(value);
    slice._length = units.length;
    slice._bytes = allocator<ffi.Uint8>(slice._length).cast();
    slice._bytes.cast<ffi.Uint8>().asTypedList(slice._length).setAll(0, units);
    return slice;
  }

  // ignore: unused_element
  String get _asDart => Utf8Decoder().convert(_bytes.cast<ffi.Uint8>().asTypedList(_length));

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfi2Utf8 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes.cast<ffi.Uint8>()[i] != _bytes.cast<ffi.Uint8>()[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

final class _SliceFfiDouble extends ffi.Struct {
  external ffi.Pointer<ffi.Double> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfiDouble _fromDart(Float64List value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfiDouble>();
    final slice = pointer.ref;
    slice._length = value.length;
    slice._bytes = allocator(slice._length);
    slice._bytes.asTypedList(slice._length).setAll(0, value);
    return slice;
  }

  // ignore: unused_element
  Float64List get _asDart => _bytes.asTypedList(_length);

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfiDouble || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

final class _SliceFfiUint16 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint16> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfiUint16 _fromDart(Uint16List value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfiUint16>();
    final slice = pointer.ref;
    slice._length = value.length;
    slice._bytes = allocator(slice._length);
    slice._bytes.asTypedList(slice._length).setAll(0, value);
    return slice;
  }

  // ignore: unused_element
  Uint16List get _asDart => _bytes.asTypedList(_length);

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfiUint16 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

final class _SliceFfiUint8 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfiUint8 _fromDart(Uint8List value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfiUint8>();
    final slice = pointer.ref;
    slice._length = value.length;
    slice._bytes = allocator(slice._length);
    slice._bytes.asTypedList(slice._length).setAll(0, value);
    return slice;
  }

  // ignore: unused_element
  Uint8List get _asDart => _bytes.asTypedList(_length);

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfiUint8 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

/// An unspecified error value
class VoidError {}

final class _Writeable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  _Writeable() : _underlying = _create(0);
  static final _create =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>>('diplomat_buffer_writeable_create')
    .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>();

  String finalize() {
    final string = _getBytes(_underlying).toDartString(length: _len(_underlying));
    _destroy(_underlying);
    return string;
  }
  static final _len = 
    _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>('diplomat_buffer_writeable_len')
    .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  static final _getBytes = 
    _capi<ffi.NativeFunction<ffi.Pointer<ffi2.Utf8> Function(ffi.Pointer<ffi.Opaque>)>>('diplomat_buffer_writeable_get_bytes')
    .asFunction<ffi.Pointer<ffi2.Utf8> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
  static final _destroy =
    _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>('diplomat_buffer_writeable_destroy')
    .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
