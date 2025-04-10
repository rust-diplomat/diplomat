// generated by diplomat-tool
// dart format off

part of 'lib.g.dart';

final class _OptionInputStructFfi extends ffi.Struct {
  external _ResultUint8Void a;
  external _ResultUint32Void b;
  external _ResultInt32Void c;
}

final class OptionInputStruct {
  int? a;
  Rune? b;
  OptionEnum? c;

  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  // ignore: unused_element
  OptionInputStruct._fromFfi(_OptionInputStructFfi ffi) :
    a = ffi.a.isOk ? ffi.a.union.ok : null,
    b = ffi.b.isOk ? ffi.b.union.ok : null,
    c = ffi.c.isOk ? OptionEnum.values[ffi.c.union.ok] : null;

  // ignore: unused_element
  _OptionInputStructFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_OptionInputStructFfi>();
    int? a = this.a;
    struct.a = a != null ? _ResultUint8Void.ok(a) : _ResultUint8Void.err();
    Rune? b = this.b;
    struct.b = b != null ? _ResultUint32Void.ok(b) : _ResultUint32Void.err();
    OptionEnum? c = this.c;
    struct.c = c != null ? _ResultInt32Void.ok(c.index) : _ResultInt32Void.err();
    return struct;
  }

  factory OptionInputStruct({int? a, Rune? b, OptionEnum? c}) {
    final result = _OptionInputStruct_default_ctor();
        final dart = OptionInputStruct._fromFfi(result);
    if (a != null) {
      dart.a = a;
    }
    if (b != null) {
      dart.b = b;
    }
    if (c != null) {
      dart.c = c;
    }
    return dart;

  }

  @override
  bool operator ==(Object other) =>
      other is OptionInputStruct &&
      other.a == a &&
      other.b == b &&
      other.c == c;

  @override
  int get hashCode => Object.hashAll([
        a,
        b,
        c,
      ]);
}

@_DiplomatFfiUse('OptionInputStruct_default_ctor')
@ffi.Native<_OptionInputStructFfi Function()>(isLeaf: true, symbol: 'OptionInputStruct_default_ctor')
// ignore: non_constant_identifier_names
external _OptionInputStructFfi _OptionInputStruct_default_ctor();

// dart format on
