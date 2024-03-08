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

final _nopFree = core.Finalizer((nothing) => {});

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
  ffi.Pointer<ffi.Uint8> allocIn(ffi.Allocator alloc) {
    return alloc<ffi.Uint8>(length)..asTypedList(length).setRange(0, length, asUint8List());
  }

  int get length => lengthInBytes;
}

extension on String {
  // ignore: unused_element
  _Utf8View get utf8View => _Utf8View(this);
  // ignore: unused_element
  _Utf16View get utf16View => _Utf16View(this);
}

extension on core.List<bool> {
  // ignore: unused_element
  _BoolListView get boolView => _BoolListView(this);
}

extension on core.List<int> {
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

extension on core.List<double> {
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

  ffi.Pointer<ffi.Uint8> allocIn(ffi.Allocator alloc) {
    // Copies
    return alloc<ffi.Uint8>(length)..asTypedList(length).setRange(0, length, _codeUnits);
  }

  int get length => _codeUnits.length;
}

// ignore: unused_element
class _Utf16View {
  final core.List<int> _codeUnits;

  _Utf16View(String string) : _codeUnits = string.codeUnits;

  ffi.Pointer<ffi.Uint16> allocIn(ffi.Allocator alloc) {
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
  ffi.Pointer<ffi.Bool> allocIn(ffi.Allocator alloc) {
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
  ffi.Pointer<ffi.Int8> allocIn(ffi.Allocator alloc) {
    return alloc<ffi.Int8>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

class _Int16ListView {
  final core.List<int> _values;

  _Int16ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Int16> allocIn(ffi.Allocator alloc) {
    return alloc<ffi.Int16>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

class _Int32ListView {
  final core.List<int> _values;

  _Int32ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Int32> allocIn(ffi.Allocator alloc) {
    return alloc<ffi.Int32>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

class _Int64ListView {
  final core.List<int> _values;

  _Int64ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Int64> allocIn(ffi.Allocator alloc) {
    return alloc<ffi.Int64>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

// ignore: unused_element
class _IsizeListView {
  final core.List<int> _values;

  _IsizeListView(this._values);

  // Copies
  ffi.Pointer<ffi.IntPtr> allocIn(ffi.Allocator alloc) {
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
  ffi.Pointer<ffi.Uint8> allocIn(ffi.Allocator alloc) {
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
  ffi.Pointer<ffi.Uint16> allocIn(ffi.Allocator alloc) {
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
  ffi.Pointer<ffi.Uint32> allocIn(ffi.Allocator alloc) {
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
  ffi.Pointer<ffi.Uint64> allocIn(ffi.Allocator alloc) {
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
  ffi.Pointer<ffi.Size> allocIn(ffi.Allocator alloc) {
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
  ffi.Pointer<ffi.Float> allocIn(ffi.Allocator alloc) {
    return alloc<ffi.Float>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}

class _Float64ListView {
  final core.List<double> _values;

  _Float64ListView(this._values);

  // ignore: unused_element
  ffi.Pointer<ffi.Double> allocIn(ffi.Allocator alloc) {
    return alloc<ffi.Double>(length)..asTypedList(length).setRange(0, length, _values);
  }

  int get length => _values.length;
}
