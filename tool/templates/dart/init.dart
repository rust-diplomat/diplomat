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

