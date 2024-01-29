// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

import 'dart:convert';
import 'dart:core' as core;
import 'dart:core' show int, double, bool, String, Object, override;
import 'dart:ffi' as ffi;
import 'dart:math';
import 'dart:typed_data';
import 'package:ffi/ffi.dart' as ffi2 show Arena, calloc;
part 'AttrEnum.g.dart';
part 'AttrOpaque1.g.dart';
part 'AttrOpaque2.g.dart';
part 'Bar.g.dart';
part 'BorrowedFields.g.dart';
part 'BorrowedFieldsReturning.g.dart';
part 'BorrowedFieldsWithBounds.g.dart';
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

/// A [Rune] is a Unicode code point, such as `a`, or `💡`.
/// 
/// The recommended way to obtain a [Rune] is to create it from a 
/// [String], which is conceptually a list of [Runes]. For example,
/// `'a'.runes.first` is equal to the [Rune] `a`.
/// 
/// Dart does not have a character/rune literal, so integer literals
/// need to be used. For example the Unicode code point U+1F4A1, `💡`,
/// can be represented by `0x1F4A1`. Note that only values in the ranges
/// `0x0..0xD7FF` and `0xE000..0x10FFFF` (both inclusive) are Unicode
/// code points, and hence valid [Rune]s.
///
/// A [String] can be constructed from a [Rune] using [String.fromCharCode]. 
typedef Rune = int;

// ignore: unused_element
final _callocFree = core.Finalizer(ffi2.calloc.free);

// ignore: unused element
final _arenaFinalizer = core.Finalizer<ffi2.Arena>((arena) => arena.releaseAll());

/// An unspecified error value
// ignore: unused_element
class VoidError {
  @override
  bool operator ==(Object other) => other is VoidError;

  @override
  int get hashCode => 1;
}

extension _View on ByteBuffer {
  // ignore: unused_element
  ffi.Pointer<ffi.Uint8> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Uint8>(length)..asTypedList(length).setRange(0, length, asUint8List());
  }

  int get length => lengthInBytes;
}

extension _UtfViews on String {
  // ignore: unused_element
  _Utf8View get utf8View => _Utf8View(this);
  // ignore: unused_element
  _Utf16View get utf16View => _Utf16View(this);
}

extension _NativeBoolViews on core.List<bool> {
  // ignore: unused_element
  _BoolListView get boolView => _BoolListView(this);
}

extension _NativeIntViews on core.List<int> {
  // ignore: unused_element
  _Int8ListView get int8View => _Int8ListView(this);
  // ignore: unused_element
  _Int16ListView get int16View => _Int16ListView(this);
  // ignore: unused_element
  _Int32ListView get int32View => _Int32ListView(this);
  // ignore: unused_element
  _Int64ListView get int64View => _Int64ListView(this);
  // ignore: unused_element
  _IsizeListView get isizeView => _IsizeListView(this);
  // ignore: unused_element
  _Uint8ListView get uint8View => _Uint8ListView(this);
  // ignore: unused_element
  _Uint16ListView get uint16View => _Uint16ListView(this);
  // ignore: unused_element
  _Uint32ListView get uint32View => _Uint32ListView(this);
  // ignore: unused_element
  _Uint64ListView get uint64View => _Uint64ListView(this);
  // ignore: unused_element
  _UsizeListView get usizeView => _UsizeListView(this);
}

extension _NativeFloatViews on core.List<double> {
  // ignore: unused_element
  _Float32ListView get float32View => _Float32ListView(this);
  // ignore: unused_element
  _Float64ListView get float64View => _Float64ListView(this);
}

// ignore: unused_element
class _Utf8View {
  final Uint8List _codeUnits;

  // Copies
  _Utf8View(String string) : _codeUnits = Utf8Encoder().convert(string);

  ffi.Pointer<ffi.Uint8> pointer(ffi.Allocator alloc) {
    // Copies
    return alloc<ffi.Uint8>(length)..asTypedList(length).setRange(0, length, _codeUnits);
  }

  int get length => _codeUnits.length;
}

// ignore: unused_element
class _Utf16View {
  final core.List<int> _codeUnits;

  _Utf16View(String string) : _codeUnits = string.codeUnits;

  ffi.Pointer<ffi.Uint16> pointer(ffi.Allocator alloc) {
    // Copies
    return alloc<ffi.Uint16>(length)..asTypedList(length).setRange(0, length, _codeUnits);
  }

  int get length => _codeUnits.length;
}

// ignore: unused_element
class _BoolListView{
  final core.List<bool> _values;

  _BoolListView(this._values);

  // Copies
  ffi.Pointer<ffi.Bool> pointer(ffi.Allocator alloc) {
    final pointer = alloc<ffi.Bool>(_values.length);
    for (var i = 0; i < _values.length; i++) {
      pointer[i] = _values[i];
    }
    return pointer;
  }

  int get length => _values.length;
}

class _Int8ListView {
  final core.List<int> _values;

  _Int8ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Int8> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Int8>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

class _Int16ListView {
  final core.List<int> _values;

  _Int16ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Int16> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Int16>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

class _Int32ListView {
  final core.List<int> _values;

  _Int32ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Int32> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Int32>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

class _Int64ListView {
  final core.List<int> _values;

  _Int64ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Int64> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Int64>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

// ignore: unused_element
class _IsizeListView {
  final core.List<int> _values;

  _IsizeListView(this._values);

  // Copies
  ffi.Pointer<ffi.IntPtr> pointer(ffi.Allocator alloc) {
    final pointer = alloc<ffi.IntPtr>(_values.length);
    for (var i = 0; i < _values.length; i++) {
      pointer[i] = _values[i];
    }
    return pointer;
  }

  int get length => _values.length;
}

class _Uint8ListView {
  final core.List<int> _values;

  _Uint8ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Uint8> pointer(ffi.Allocator alloc) {
    final pointer = alloc<ffi.Uint8>(_values.length);
    for (var i = 0; i < _values.length; i++) {
      pointer[i] = min(255, max(0, _values[i]));
    }
    return pointer;
  }

  int get length => _values.length;
}

class _Uint16ListView {
  final core.List<int> _values;

  _Uint16ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Uint16> pointer(ffi.Allocator alloc) {
    final pointer = alloc<ffi.Uint16>(_values.length);
    for (var i = 0; i < _values.length; i++) {
      pointer[i] = min(65535, max(0, _values[i]));
    }
    return pointer;
  }

  int get length => _values.length;
}

class _Uint32ListView {
  final core.List<int> _values;

  _Uint32ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Uint32> pointer(ffi.Allocator alloc) {
    final pointer = alloc<ffi.Uint32>(_values.length);
    for (var i = 0; i < _values.length; i++) {
      pointer[i] = min(4294967295, max(0, _values[i]));
    }
    return pointer;
  }

  int get length => _values.length;
}

class _Uint64ListView {
  final core.List<int> _values;

  _Uint64ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Uint64> pointer(ffi.Allocator alloc) {
    final pointer = alloc<ffi.Uint64>(_values.length);
    for (var i = 0; i < _values.length; i++) {
      pointer[i] = max(0, _values[i]);
    }
    return pointer;
  }

  int get length => _values.length;
}

// ignore: unused_element
class _UsizeListView {
  final core.List<int> _values;

  _UsizeListView(this._values);

  // Copies
  ffi.Pointer<ffi.Size> pointer(ffi.Allocator alloc) {
    final pointer = alloc<ffi.Size>(_values.length);
    for (var i = 0; i < _values.length; i++) {
      pointer[i] = max(0, _values[i]);
    }
    return pointer;
  }

  int get length => _values.length;
}

class _Float32ListView {
  final core.List<double> _values;

  _Float32ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Float> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Float>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

class _Float64ListView {
  final core.List<double> _values;

  _Float64ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Double> pointer(ffi.Allocator alloc) {
    return alloc<ffi.Double>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

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

final class _SliceUtf16 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint16> _pointer;

  @ffi.Size()
  external int _length;

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceUtf16 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._pointer[i] != _pointer[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

final class _SliceUtf8 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> _pointer;

  @ffi.Size()
  external int _length;

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceUtf8 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._pointer[i] != _pointer[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

final class _Writeable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  _Writeable() : _underlying = _diplomat_buffer_writeable_create(0);
  
  String finalize() {
    final string = Utf8Decoder().convert(_diplomat_buffer_writeable_get_bytes(_underlying).asTypedList(_diplomat_buffer_writeable_len(_underlying)));
    _diplomat_buffer_writeable_destroy(_underlying);
    return string;
  }
}

  
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>(symbol: 'diplomat_buffer_writeable_create', isLeaf: true)
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _diplomat_buffer_writeable_create(int len);

@ffi.Native<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_writeable_len', isLeaf: true)
// ignore: non_constant_identifier_names
external int _diplomat_buffer_writeable_len(ffi.Pointer<ffi.Opaque> ptr);

@ffi.Native<ffi.Pointer<ffi.Uint8> Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_writeable_get_bytes', isLeaf: true)
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Uint8> _diplomat_buffer_writeable_get_bytes(ffi.Pointer<ffi.Opaque> ptr);

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_writeable_destroy', isLeaf: true)
// ignore: non_constant_identifier_names
external void _diplomat_buffer_writeable_destroy(ffi.Pointer<ffi.Opaque> ptr);
