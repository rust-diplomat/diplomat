
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
/// A list of [Rune]s.
typedef RuneList = Uint32List;

late final ffi.Pointer<T> Function<T extends ffi.NativeType>(String) _capi;
void init(String path) => _capi = ffi.DynamicLibrary.open(path).lookup;

final _callocFree = core.Finalizer(ffi2.calloc.free);

extension _AllocConvert on Utf8Encoder {
  ffi.Pointer<ffi.Uint8> allocConvert(ffi.Allocator alloc, String string, {int? length}) {
      final l = length ?? string.utf8Length;
      return alloc<ffi.Uint8>(l)..asTypedList(l).setAll(0, convert(string));
   }
}

extension _Utf8Length on String {
  int get utf8Length {
    var length = 0;
    for (var rune in runes) {
      if (rune < 0x80) {
        length += 1;
      } else if (rune < 0x800) {
        length += 2;
      } else if (rune < 0x10000) {
        length += 3;
      } else {
        length += 4;
      }
    }
    return length;
  }
}

extension _CopyString on String {
  ffi.Pointer<ffi.Uint16> copy(ffi.Allocator alloc) {
    return alloc<ffi.Uint16>(length)..asTypedList(length).setAll(0, codeUnits);
  }
}

extension _CopyInt8List on Int8List {
  ffi.Pointer<ffi.Int8> copy(ffi.Allocator alloc) {
    return alloc<ffi.Int8>(length)..asTypedList(length).setAll(0, this);
  }
}

extension _CopyInt16List on Int16List {
  ffi.Pointer<ffi.Int16> copy(ffi.Allocator alloc) {
    return alloc<ffi.Int16>(length)..asTypedList(length).setAll(0, this);
  }
}

extension _CopyInt32List on Int32List {
  ffi.Pointer<ffi.Int32> copy(ffi.Allocator alloc) {
    return alloc<ffi.Int32>(length)..asTypedList(length).setAll(0, this);
  }
}

extension _CopyInt64List on Int64List {
  ffi.Pointer<ffi.Int64> copy(ffi.Allocator alloc) {
    return alloc<ffi.Int64>(length)..asTypedList(length).setAll(0, this);
  }
}

extension _CopyUint8List on Uint8List {
  ffi.Pointer<ffi.Uint8> copy(ffi.Allocator alloc) {
    return alloc<ffi.Uint8>(length)..asTypedList(length).setAll(0, this);
  }
}

extension _CopyUint16List on Uint16List {
  ffi.Pointer<ffi.Uint16> copy(ffi.Allocator alloc) {
    return alloc<ffi.Uint16>(length)..asTypedList(length).setAll(0, this);
  }
}

extension _CopyUint32List on Uint32List {
  ffi.Pointer<ffi.Uint32> copy(ffi.Allocator alloc) {
    return alloc<ffi.Uint32>(length)..asTypedList(length).setAll(0, this);
  }
}

extension _CopyUint64List on Uint64List {
  ffi.Pointer<ffi.Uint64> copy(ffi.Allocator alloc) {
    return alloc<ffi.Uint64>(length)..asTypedList(length).setAll(0, this);
  }
}

extension _CopyFloat32List on Float32List {
  ffi.Pointer<ffi.Float> copy(ffi.Allocator alloc) {
    return alloc<ffi.Float>(length)..asTypedList(length).setAll(0, this);
  }
}

extension _CopyFloat64List on Float64List {
  ffi.Pointer<ffi.Double> copy(ffi.Allocator alloc) {
    return alloc<ffi.Double>(length)..asTypedList(length).setAll(0, this);
  }
}
