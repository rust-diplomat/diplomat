// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _BorrowedFieldsFfi extends ffi.Struct {
  external _SliceUtf16 a;
  external _SliceUtf8 b;
}

final class BorrowedFields {
  final _BorrowedFieldsFfi _underlying;

  BorrowedFields._(this._underlying);

  factory BorrowedFields() {
    final pointer = ffi2.calloc<_BorrowedFieldsFfi>();
    final result = BorrowedFields._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  String get a => core.String.fromCharCodes(_underlying.a._pointer.asTypedList(_underlying.a._length));
  set a(String a) {
    ffi2.calloc.free(_underlying.a._pointer);
    _underlying.a._length = a.length;
    _underlying.a._pointer = a.copy(ffi2.calloc);
  }

  String get b => Utf8Decoder().convert(_underlying.b._pointer.asTypedList(_underlying.b._length));
  set b(String b) {
    ffi2.calloc.free(_underlying.b._pointer);
    _underlying.b._length = b.utf8Length;
    _underlying.b._pointer = Utf8Encoder().allocConvert(ffi2.calloc, b, length: _underlying.b._length);
  }

  @override
  bool operator ==(Object other) =>
      other is BorrowedFields &&
      other._underlying.a == _underlying.a &&
      other._underlying.b == _underlying.b;

  @override
  int get hashCode => Object.hashAll([
        _underlying.a,
        _underlying.b,
      ]);
}
