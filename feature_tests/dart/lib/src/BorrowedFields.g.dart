// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _BorrowedFieldsFfi extends ffi.Struct {
  external _SliceUtf16 a;
  external _SliceUtf8 b;
  external _SliceUtf8 c;
}

final class BorrowedFields {
  String _a;
  String _b;
  String _c;

  BorrowedFields._(this._a,this._b,this._c,);


  factory BorrowedFields._fromFfi(_BorrowedFieldsFfi ffi){

    var _underlying = ffi;
    var _a = core.String.fromCharCodes(_underlying.a._pointer.asTypedList(_underlying.a._length));
    var _b = Utf8Decoder().convert(_underlying.b._pointer.asTypedList(_underlying.b._length));
    var _c = Utf8Decoder().convert(_underlying.c._pointer.asTypedList(_underlying.c._length));
    return BorrowedFields._(_a, _b, _c, );
  }

  _BorrowedFieldsFfi _toFfi() {
    final pointer = ffi2.calloc<_BorrowedFieldsFfi>();
    var _underlying = pointer.ref;
    ffi2.calloc.free(_underlying.a._pointer);;
    final aView = a.utf16View;;
    _underlying.a._pointer = aView.pointer(ffi2.calloc);;
    _underlying.a._length = aView.length;;
    ffi2.calloc.free(_underlying.b._pointer);;
    final bView = b.utf8View;;
    _underlying.b._pointer = bView.pointer(ffi2.calloc);;
    _underlying.b._length = bView.length;;
    ffi2.calloc.free(_underlying.c._pointer);;
    final cView = c.utf8View;;
    _underlying.c._pointer = cView.pointer(ffi2.calloc);;
    _underlying.c._length = cView.length;;

    _callocFree.attach(_underlying, pointer.cast());
    return _underlying;
  }

  String get a => this._a;
  set a(String a) {
    _a = a;
  }

  String get b => this._b;
  set b(String b) {
    _b = b;
  }

  String get c => this._c;
  set c(String c) {
    _c = c;
  }

  @override
  bool operator ==(Object other) =>
      other is BorrowedFields &&
      other.a == this.a &&
      other.b == this.b &&
      other.c == this.c;

  @override
  int get hashCode => Object.hashAll([
        this.a,
        this.b,
        this.c,
      ]);
}
