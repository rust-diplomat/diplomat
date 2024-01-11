// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _OptionStructFfi extends ffi.Struct {
  external ffi.Pointer<ffi.Opaque> a;
  external ffi.Pointer<ffi.Opaque> b;
  @ffi.Uint32()
  external int c;
  external ffi.Pointer<ffi.Opaque> d;
}

final class OptionStruct {
  final OptionOpaque? _a;
  final OptionOpaqueChar? _b;
  final int _c;
  final OptionOpaque? _d;

  OptionStruct._(this._a,this._b,this._c,this._d,);


  factory OptionStruct._fromFfi(_OptionStructFfi ffi){

    var _underlying = ffi;
    var _a = _underlying.a.address == 0 ? null : OptionOpaque._(_underlying.a, true);
    var _b = _underlying.b.address == 0 ? null : OptionOpaqueChar._(_underlying.b, true);
    var _c = _underlying.c;
    var _d = _underlying.d.address == 0 ? null : OptionOpaque._(_underlying.d, true);
    return OptionStruct._(_a, _b, _c, _d, );
  }

  _OptionStructFfi _toFfi() {
    final pointer = ffi2.calloc<_OptionStructFfi>();
    var _underlying = pointer.ref;

    _callocFree.attach(_underlying, pointer.cast());
    return _underlying;
  }

  OptionOpaque? get a => this._a;

  OptionOpaqueChar? get b => this._b;

  int get c => this._c;

  OptionOpaque? get d => this._d;

  @override
  bool operator ==(Object other) =>
      other is OptionStruct &&
      other.a == this.a &&
      other.b == this.b &&
      other.c == this.c &&
      other.d == this.d;

  @override
  int get hashCode => Object.hashAll([
        this.a,
        this.b,
        this.c,
        this.d,
      ]);
}
