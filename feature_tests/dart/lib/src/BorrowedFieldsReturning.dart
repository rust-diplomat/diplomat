import 'dart:ffi' as ffi;
import 'dart:typed_data';
import 'diplomat_runtime.dart';
import 'package:ffi/ffi.dart' as allocators;

class BorrowedFieldsReturningFfi extends ffi.Struct {
  external Slice bytes;
}

class BorrowedFieldsReturning {
  final BorrowedFieldsReturningFfi _underlying;
  Uint8List get bytes => this
      ._underlying
      .bytes
      .bytes
      .cast<ffi.Uint8>()
      .asTypedList(this._underlying.bytes.length);
  void set bytes(Uint8List bytes) {
    final alloc = allocators.calloc;
    alloc.free(this._underlying.bytes.bytes);
    final bytesBytes = alloc.call<ffi.Uint8>(bytes.length);
    bytesBytes.asTypedList(bytes.length).setAll(0, bytes);
    this._underlying.bytes.bytes = bytesBytes.cast();
    this._underlying.bytes.length = bytes.length;
  }

  BorrowedFieldsReturning._(this._underlying);
}

// These are not methods because we want to keep them package-private, and methods are either private or public
BorrowedFieldsReturning BorrowedFieldsReturningFromFfi(
        BorrowedFieldsReturningFfi underlying) =>
    BorrowedFieldsReturning._(underlying);
BorrowedFieldsReturningFfi BorrowedFieldsReturningAsFfi(
        BorrowedFieldsReturning t) =>
    t._underlying;
