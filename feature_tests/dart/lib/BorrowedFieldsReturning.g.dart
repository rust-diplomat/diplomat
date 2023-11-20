// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _BorrowedFieldsReturningFfi extends ffi.Struct {
  external _SliceFfiUint8 bytes;
}

final class BorrowedFieldsReturning {
  final _BorrowedFieldsReturningFfi _underlying;

  // ignore: unused_element
  BorrowedFieldsReturning._(this._underlying);

  factory BorrowedFieldsReturning() {
    final pointer = ffi2.calloc<_BorrowedFieldsReturningFfi>();
    final result = BorrowedFieldsReturning._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  Uint8List get bytes => _underlying.bytes._asDart;
  set bytes(Uint8List bytes) {
    final alloc = ffi2.calloc;
    alloc.free(_underlying.bytes._bytes);
    final bytesSlice = _SliceFfiUint8._fromDart(bytes, alloc);
    _underlying.bytes = bytesSlice;
  }

  @override
  bool operator ==(Object other) =>
      other is BorrowedFieldsReturning &&
      other._underlying.bytes == _underlying.bytes;

  @override
  int get hashCode => Object.hashAll([
        _underlying.bytes,
      ]);
}
