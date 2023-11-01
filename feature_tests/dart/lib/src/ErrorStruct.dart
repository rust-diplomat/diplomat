import 'dart:ffi' as ffi;

class ErrorStructFfi extends ffi.Struct {
  @ffi.Int32()
  external int i;
  @ffi.Int32()
  external int j;
}

class ErrorStruct {
  final ErrorStructFfi _underlying;
  int get i => this._underlying.i;
  void set i(int i) {
    this._underlying.i = i;
  }

  int get j => this._underlying.j;
  void set j(int j) {
    this._underlying.j = j;
  }

  ErrorStruct._(this._underlying);
}

// These are not methods because we want to keep them package-private, and methods are either private or public
ErrorStruct ErrorStructFromFfi(ErrorStructFfi underlying) =>
    ErrorStruct._(underlying);
ErrorStructFfi ErrorStructAsFfi(ErrorStruct t) => t._underlying;
