// generated by diplomat-tool

part of 'lib.g.dart';

final class _ErrorStructFfi extends ffi.Struct {
  @ffi.Int32()
  external int i;
  @ffi.Int32()
  external int j;
}

final class ErrorStruct {
  int i;
  int j;

  ErrorStruct({required this.i, required this.j});

  // Internal constructor from FFI.
  // ignore: unused_element
  ErrorStruct._(_ErrorStructFfi underlying) :
    i = underlying.i,
    j = underlying.j;

  // ignore: unused_element
  _ErrorStructFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_ErrorStructFfi>();
    struct.i = i;
    struct.j = j;
    return struct;
  }

  @override
  bool operator ==(Object other) =>
      other is ErrorStruct &&
      other.i == this.i &&
      other.j == this.j;

  @override
  int get hashCode => Object.hashAll([
        this.i,
        this.j,
      ]);
}
