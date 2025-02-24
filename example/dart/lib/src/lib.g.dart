// generated by diplomat-tool
// dart format off

// ignore: unused_import
import 'dart:core' as core;
// ignore: unused_import
import 'dart:typed_data';
// ignore: unused_shown_name
import 'dart:core' show int, double, bool, String, Object, override;
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'package:ffi/ffi.dart' as ffi2 show Arena, calloc;
import 'package:meta/meta.dart' as meta;
part 'DataProvider.g.dart';
part 'FixedDecimal.g.dart';
part 'FixedDecimalFormatter.g.dart';
part 'FixedDecimalFormatterOptions.g.dart';
part 'FixedDecimalGroupingStrategy.g.dart';
part 'Locale.g.dart';

class _DiplomatFfiUse extends meta.RecordUse {
  final String symbol;

  const _DiplomatFfiUse(@meta.mustBeConst this.symbol);
}

/// A [Rune] is a Unicode code point, such as `a`, or `💡`.
/// 
/// The recommended way to obtain a [Rune] is to create it from a 
/// [String], which is conceptually a sequence of [Rune]s. For
/// example, `'a'.runes.first` is equal to the [Rune] `a`.
/// 
/// Dart does not have a character/rune literal (https://github.com/dart-lang/language/issues/886),
/// so integer literals need to be used. For example the Unicode code point 
/// U+1F4A1, `💡`, can be represented by `0x1F4A1`.
///
/// A [String] can be constructed from a [Rune] using (the [confusingly named](
/// https://github.com/dart-lang/sdk/issues/56304)) [String.fromCharCode]. 
typedef Rune = int;

// ignore: unused_element
final _callocFree = core.Finalizer(ffi2.calloc.free);

// ignore: unused_element
final _nopFree = core.Finalizer((nothing) => {});

// ignore: unused_element
final _rustFree = core.Finalizer((({ffi.Pointer<ffi.Void> pointer, int bytes, int align}) record) => _diplomat_free(record.pointer, record.bytes, record.align));

// ignore: unused_element
final class _RustAlloc implements ffi.Allocator {
  @override
  ffi.Pointer<T> allocate<T extends ffi.NativeType>(int byteCount, {int? alignment}) {
      return _diplomat_alloc(byteCount, alignment ?? 1).cast();
  }

  @override
  void free(ffi.Pointer<ffi.NativeType> pointer) {
    throw 'Internal error: should not deallocate in Rust memory';
  }
}

@_DiplomatFfiUse('diplomat_alloc')
@ffi.Native<ffi.Pointer<ffi.Void> Function(ffi.Size, ffi.Size)>(symbol: 'diplomat_alloc', isLeaf: true)
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Void> _diplomat_alloc(int len, int align);

@_DiplomatFfiUse('diplomat_free')
@ffi.Native<ffi.Size Function(ffi.Pointer<ffi.Void>, ffi.Size, ffi.Size)>(symbol: 'diplomat_free', isLeaf: true)
// ignore: non_constant_identifier_names
external int _diplomat_free(ffi.Pointer<ffi.Void> ptr, int len, int align);


// ignore: unused_element
class _FinalizedArena {
  final ffi2.Arena arena;
  static final core.Finalizer<ffi2.Arena> _finalizer = core.Finalizer((arena) => arena.releaseAll());

  // ignore: unused_element
  _FinalizedArena() : arena = ffi2.Arena() {
    _finalizer.attach(this, arena);
  }

  // ignore: unused_element
  _FinalizedArena.withLifetime(core.List<core.List<Object>> lifetimeAppendArray) : arena = ffi2.Arena() {
    _finalizer.attach(this, arena);
    for (final edge in lifetimeAppendArray) {
      edge.add(this);
    }
  }
}

final class _ResultOpaqueVoidUnion extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> ok;

}

final class _ResultOpaqueVoid extends ffi.Struct {
  external _ResultOpaqueVoidUnion union;

  @ffi.Bool()
  external bool isOk;

  // ignore: unused_element
  factory _ResultOpaqueVoid.ok(ffi.Pointer<ffi.Opaque> val) {
    final struct = ffi.Struct.create<_ResultOpaqueVoid>();
    struct.isOk = true;
    struct.union.ok = val;
    return struct;
  }
  // ignore: unused_element
  factory _ResultOpaqueVoid.err() {
    final struct = ffi.Struct.create<_ResultOpaqueVoid>();
    struct.isOk = false;
    return struct;
  }
}

final class _ResultVoidVoid extends ffi.Struct {
  

  @ffi.Bool()
  external bool isOk;

  // ignore: unused_element
  factory _ResultVoidVoid.ok() {
    final struct = ffi.Struct.create<_ResultVoidVoid>();
    struct.isOk = true;
    return struct;
  }
  // ignore: unused_element
  factory _ResultVoidVoid.err() {
    final struct = ffi.Struct.create<_ResultVoidVoid>();
    struct.isOk = false;
    return struct;
  }
}

final class _SliceUtf8 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> _data;

  @ffi.Size()
  external int _length;

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceUtf8 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._data[i] != _data[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;

  // ignore: unused_element
  String _toDart(core.List<Object> lifetimeEdges) {
    final r = Utf8Decoder().convert(_data.asTypedList(_length));
    if (lifetimeEdges.isEmpty) {
      _diplomat_free(_data.cast(), _length, 1);
    } else {
      // Lifetime edges will be cleaned up
    }
    return r;
  }
}

extension on String {
  // ignore: unused_element
  _SliceUtf8 _utf8AllocIn(ffi.Allocator alloc) {
    final slice = ffi.Struct.create<_SliceUtf8>();
    final encoded = Utf8Encoder().convert(this);
    slice._data = alloc(encoded.length)..asTypedList(encoded.length).setRange(0, encoded.length, encoded);
    slice._length = encoded.length;
    return slice;
  }
}

final class _Write {
  final ffi.Pointer<ffi.Opaque> _ffi;

  _Write() : _ffi = _diplomat_buffer_write_create(0);
  
  String finalize() {
    try {
      final buf = _diplomat_buffer_write_get_bytes(_ffi);
      if (buf == ffi.Pointer.fromAddress(0)) {
        throw core.OutOfMemoryError();
      }
      return Utf8Decoder().convert(buf.asTypedList(_diplomat_buffer_write_len(_ffi)));
    } finally {
      _diplomat_buffer_write_destroy(_ffi);
    }
  }
}

@_DiplomatFfiUse('diplomat_buffer_write_create')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>(symbol: 'diplomat_buffer_write_create', isLeaf: true)
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _diplomat_buffer_write_create(int len);

@_DiplomatFfiUse('diplomat_buffer_write_len')
@ffi.Native<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_write_len', isLeaf: true)
// ignore: non_constant_identifier_names
external int _diplomat_buffer_write_len(ffi.Pointer<ffi.Opaque> ptr);

@_DiplomatFfiUse('diplomat_buffer_write_get_bytes')
@ffi.Native<ffi.Pointer<ffi.Uint8> Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_write_get_bytes', isLeaf: true)
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Uint8> _diplomat_buffer_write_get_bytes(ffi.Pointer<ffi.Opaque> ptr);

@_DiplomatFfiUse('diplomat_buffer_write_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(symbol: 'diplomat_buffer_write_destroy', isLeaf: true)
// ignore: non_constant_identifier_names
external void _diplomat_buffer_write_destroy(ffi.Pointer<ffi.Opaque> ptr);

// dart format on
