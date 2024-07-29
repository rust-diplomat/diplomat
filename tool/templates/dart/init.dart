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

final class _RustAlloc implements ffi.Allocator {
  @override
  ffi.Pointer<T> allocate<T extends ffi.NativeType>(int byteCount, {int? alignment}) {
      return _diplomat_alloc(byteCount, alignment ?? 1).cast();
  }

  void free(ffi.Pointer<ffi.NativeType> pointer) {
    throw 'Internal error: should not deallocate in Rust memory';
  }
}

@meta.ResourceIdentifier('diplomat_alloc')
@ffi.Native<ffi.Pointer<ffi.Void> Function(ffi.Size, ffi.Size)>(symbol: 'diplomat_alloc', isLeaf: true)
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Void> _diplomat_alloc(int len, int align);

@meta.ResourceIdentifier('diplomat_free')
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

extension on ByteBuffer {
  // ignore: unused_element
  (ffi.Pointer<ffi.Uint8>, int) _rawBytesAllocIn(ffi.Allocator alloc) => (alloc<ffi.Uint8>(lengthInBytes)..asTypedList(lengthInBytes).setRange(0, lengthInBytes, asUint8List()), lengthInBytes);
}

extension on String {
  // ignore: unused_element
  (ffi.Pointer<ffi.Uint8>, int) _utf8AllocIn(ffi.Allocator alloc) {
    final encoded = Utf8Encoder().convert(this);
    return (alloc<ffi.Uint8>(encoded.length)..asTypedList(encoded.length).setRange(0, encoded.length, encoded), length);
  }
}

extension on String {
  // ignore: unused_element
  (ffi.Pointer<ffi.Uint16>, int) _utf16AllocIn(ffi.Allocator alloc) => (alloc<ffi.Uint16>(codeUnits.length)..asTypedList(codeUnits.length).setRange(0, codeUnits.length, codeUnits), length);
}

extension on core.List<String> {
  // ignore: unused_element
  (ffi.Pointer<_SliceUtf8>, int) _utf8SliceAllocIn(ffi.Allocator alloc) {
    final slice = alloc<_SliceUtf8>(length);
    for (var i = 0; i < length; i++) {
      final (data, length) = this[i]._utf8AllocIn(alloc);
      slice[i]._data = data;
      slice[i]._length = length;
    }
    return (slice, length);
  }
}

extension on core.List<String> {
  // ignore: unused_element
  (ffi.Pointer<_SliceUtf16>, int) _utf16SliceAllocIn(ffi.Allocator alloc) {
    final slice = alloc<_SliceUtf16>(length);
    for (var i = 0; i < length; i++) {
      final (data, length) = this[i]._utf16AllocIn(alloc);
      slice[i]._data = data;
      slice[i]._length = length;
    }
    return (slice, length);
  }
}

extension on core.List<bool> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Bool>, int) _boolAllocIn(ffi.Allocator alloc) {
    final data = alloc<ffi.Bool>(length);
    for (var i = 0; i < length; i++) {
      data[i] = this[i];
    }
    return (data, length);
  }
}

extension on core.List<int> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Int8>, int) _int8AllocIn(ffi.Allocator alloc) => (alloc<ffi.Int8>(length)..asTypedList(length).setRange(0, length, this), length);
}

extension on core.List<int> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Int16>, int) _int16AllocIn(ffi.Allocator alloc) => (alloc<ffi.Int16>(length)..asTypedList(length).setRange(0, length, this), length);
}

extension on core.List<int> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Int32>, int) _int32AllocIn(ffi.Allocator alloc) => (alloc<ffi.Int32>(length)..asTypedList(length).setRange(0, length, this), length);
}

extension on core.List<int> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Int64>, int) _int64AllocIn(ffi.Allocator alloc) => (alloc<ffi.Int64>(length)..asTypedList(length).setRange(0, length, this), length);
}

extension on core.List<int> {
  // ignore: unused_element
  (ffi.Pointer<ffi.IntPtr>, int) _isizeAllocIn(ffi.Allocator alloc) {
    final data = alloc<ffi.IntPtr>(length);
    for (var i = 0; i < length; i++) {
      data[i] = this[i];
    }
    return (data, length);
  }
}

extension on core.List<int> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Uint8>, int) _uint8AllocIn(ffi.Allocator alloc) {
    final data = alloc<ffi.Uint8>(length);
    for (var i = 0; i < length; i++) {
      data[i] = min(255, max(0, this[i]));
    }
    return (data, length);
  }
}

extension on core.List<int> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Uint16>, int) _uint16AllocIn(ffi.Allocator alloc) {
    final data = alloc<ffi.Uint16>(length);
    for (var i = 0; i < length; i++) {
      data[i] = min(65535, max(0, this[i]));
    }
    return (data, length);
  }
}

extension on core.List<int> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Uint32>, int) _uint32AllocIn(ffi.Allocator alloc) {
    final data = alloc<ffi.Uint32>(length);
    for (var i = 0; i < length; i++) {
      data[i] = min(4294967295, max(0, this[i]));
    }
    return (data, length);
  }
}

extension on core.List<int> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Uint64>, int) _uint64AllocIn(ffi.Allocator alloc) {
    final data = alloc<ffi.Uint64>(length);
    for (var i = 0; i < length; i++) {
      data[i] = max(0, this[i]);
    }
    return (data, length);
  }
}

extension on core.List<int> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Size>, int) _usizeAllocIn(ffi.Allocator alloc) {
    final data = alloc<ffi.Size>(length);
    for (var i = 0; i < length; i++) {
      data[i] = max(0, this[i]);
    }
    return (data, length);
  }
}

extension on core.List<double> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Float>, int) _float32AllocIn(ffi.Allocator alloc) => (alloc<ffi.Float>(length)..asTypedList(length).setRange(0, length, this), length);
}

extension on core.List<double> {
  // ignore: unused_element
  (ffi.Pointer<ffi.Double>, int) _float64AllocIn(ffi.Allocator alloc) => (alloc<ffi.Double>(length)..asTypedList(length).setRange(0, length, this), length);
}
