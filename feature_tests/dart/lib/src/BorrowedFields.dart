import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:typed_data';
import 'diplomat_runtime.dart';
import 'package:ffi/ffi.dart' as allocators;

class BorrowedFieldsFfi extends ffi.Struct {
  external Slice a;
  external Slice b;
}

class BorrowedFields {
  final BorrowedFieldsFfi _underlying;
  Uint16List get a => this
      ._underlying
      .a
      .bytes
      .cast<ffi.Uint16>()
      .asTypedList(this._underlying.a.length);
  void set a(Uint16List a) {
    final alloc = allocators.calloc;
    alloc.free(this._underlying.a.bytes);
    final aBytes = alloc.call<ffi.Uint16>(a.length);
    aBytes.asTypedList(a.length).setAll(0, a);
    this._underlying.a.bytes = aBytes.cast();
    this._underlying.a.length = a.length;
  }

  String get b => Utf8Decoder(allowMalformed: false).convert(this
      ._underlying
      .b
      .bytes
      .cast<ffi.Uint8>()
      .asTypedList(this._underlying.b.length));
  void set b(String b) {
    final alloc = allocators.calloc;
    alloc.free(this._underlying.b.bytes);
    final bList = Utf8Encoder().convert(b);
    final bBytes = alloc.call<ffi.Char>(bList.length);
    bBytes.cast<ffi.Uint8>().asTypedList(bList.length).setAll(0, bList);
    this._underlying.b.bytes = bBytes.cast();
    this._underlying.b.length = bList.length;
  }

  BorrowedFields._(this._underlying);
}

// These are not methods because we want to keep them package-private, and methods are either private or public
BorrowedFields BorrowedFieldsFromFfi(BorrowedFieldsFfi underlying) =>
    BorrowedFields._(underlying);
BorrowedFieldsFfi BorrowedFieldsAsFfi(BorrowedFields t) => t._underlying;
