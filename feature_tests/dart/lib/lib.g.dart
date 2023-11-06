// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:typed_data';
import 'package:ffi/ffi.dart' as ffi2;

late final ffi.Pointer<T> Function<T extends ffi.NativeType>(String) _capi;
void init(String path) => _capi = ffi.DynamicLibrary.open(path).lookup;

final _callocFree = Finalizer(ffi2.calloc.free);

enum AttrEnum {
  a.__(0),
  b.__(1),
  c.__(2);

  const AttrEnum.__(this._id);

  // ignore: unused_element
  factory AttrEnum._(int id) => values.firstWhere((value) => value._id == id);

  final int _id;
}

class AttrOpaque1 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  AttrOpaque1._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('AttrOpaque1_destroy'));

  void method() {
    _methodFfi(_underlying);
  }

  static final _methodFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'AttrOpaque1_method')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  void methodDisabledcpp() {
    _methodDisabledcppFfi(_underlying);
  }

  static final _methodDisabledcppFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'AttrOpaque1_method_disabledcpp')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

class AttrOpaque2 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  AttrOpaque2._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('AttrOpaque2_destroy'));
}

class Bar implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Bar._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('Bar_destroy'));
}

class _BorrowedFieldsFfi extends ffi.Struct {
  external _SliceFfiUint16 a;
  external _SliceFfi2Utf8 b;
}

class BorrowedFields {
  final _BorrowedFieldsFfi _underlying;

  // ignore: unused_element
  BorrowedFields._(this._underlying);

  factory BorrowedFields() {
    final pointer = ffi2.calloc<_BorrowedFieldsFfi>();
    final result = BorrowedFields._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  Uint16List get a => _underlying.a._asDart;
  set a(Uint16List a) {
    final alloc = ffi2.calloc;
    alloc.free(_underlying.a._bytes);
    final aSlice = _SliceFfiUint16._fromDart(a, alloc);
    _underlying.a = aSlice;
  }

  String get b => _underlying.b._asDart;
  set b(String b) {
    final alloc = ffi2.calloc;
    alloc.free(_underlying.b._bytes);
    final bSlice = _SliceFfi2Utf8._fromDart(b, alloc);
    _underlying.b = bSlice;
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

class _BorrowedFieldsReturningFfi extends ffi.Struct {
  external _SliceFfiUint8 bytes;
}

class BorrowedFieldsReturning {
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

enum ErrorEnum {
  foo.__(0),
  bar.__(1);

  const ErrorEnum.__(this._id);

  // ignore: unused_element
  factory ErrorEnum._(int id) => values.firstWhere((value) => value._id == id);

  final int _id;
}

class _ErrorStructFfi extends ffi.Struct {
  @ffi.Int32()
  external int i;
  @ffi.Int32()
  external int j;
}

class ErrorStruct {
  final _ErrorStructFfi _underlying;

  // ignore: unused_element
  ErrorStruct._(this._underlying);

  factory ErrorStruct() {
    final pointer = ffi2.calloc<_ErrorStructFfi>();
    final result = ErrorStruct._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  int get i => _underlying.i;
  set i(int i) {
    _underlying.i = i;
  }

  int get j => _underlying.j;
  set j(int j) {
    _underlying.j = j;
  }

  @override
  bool operator ==(Object other) =>
      other is ErrorStruct &&
      other._underlying.i == _underlying.i &&
      other._underlying.j == _underlying.j;

  @override
  int get hashCode => Object.hashAll([
        _underlying.i,
        _underlying.j,
      ]);
}

class Float64Vec implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Float64Vec._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('Float64Vec_destroy'));

  factory Float64Vec(Float64List v) {
    final alloc = ffi2.Arena();
    final vSlice = _SliceFfiDouble._fromDart(v, alloc);

    final result = _newFfi(vSlice._bytes, vSlice._length);
    alloc.releaseAll();
    return Float64Vec._(result);
  }
  static final _newFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Double>, ffi.Size)>>('Float64Vec_new')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Double>, int)>(isLeaf: true);

  void fillSlice(Float64List v) {
    final alloc = ffi2.Arena();
    final vSlice = _SliceFfiDouble._fromDart(v, alloc);

    _fillSliceFfi(_underlying, vSlice._bytes, vSlice._length);
    alloc.releaseAll();
  }

  static final _fillSliceFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Double>, ffi.Size)>>('Float64Vec_fill_slice')
      .asFunction<
          void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Double>,
              int)>(isLeaf: true);

  void setValue(Float64List newSlice) {
    final alloc = ffi2.Arena();
    final newSliceSlice = _SliceFfiDouble._fromDart(newSlice, alloc);

    _setValueFfi(_underlying, newSliceSlice._bytes, newSliceSlice._length);
    alloc.releaseAll();
  }

  static final _setValueFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Double>, ffi.Size)>>('Float64Vec_set_value')
      .asFunction<
          void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Double>,
              int)>(isLeaf: true);
}

class Foo implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Foo._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('Foo_destroy'));

  factory Foo(String x) {
    final alloc = ffi2.Arena();
    final xSlice = _SliceFfi2Utf8._fromDart(x, alloc);

    final result = _newFfi(xSlice._bytes, xSlice._length);
    alloc.releaseAll();
    return Foo._(result);
  }
  static final _newFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi2.Utf8>, ffi.Size)>>('Foo_new')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  Bar get getBar {
    final result = _getBarFfi(_underlying);
    return Bar._(result);
  }

  static final _getBarFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('Foo_get_bar')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  factory Foo.static(String x) {
    final alloc = ffi2.Arena();
    final xSlice = _SliceFfi2Utf8._fromDart(x, alloc);

    final result = _newStaticFfi(xSlice._bytes, xSlice._length);
    alloc.releaseAll();
    return Foo._(result);
  }
  static final _newStaticFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi2.Utf8>, ffi.Size)>>('Foo_new_static')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  BorrowedFieldsReturning get asReturning {
    final result = _asReturningFfi(_underlying);
    return BorrowedFieldsReturning._(result);
  }

  static final _asReturningFfi = _capi<
          ffi.NativeFunction<
              _BorrowedFieldsReturningFfi Function(
                  ffi.Pointer<ffi.Opaque>)>>('Foo_as_returning')
      .asFunction<
          _BorrowedFieldsReturningFfi Function(
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory Foo.extractFromFields(BorrowedFields fields) {
    final result = _extractFromFieldsFfi(fields._underlying);
    return Foo._(result);
  }
  static final _extractFromFieldsFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  _BorrowedFieldsFfi)>>('Foo_extract_from_fields')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(_BorrowedFieldsFfi)>(
          isLeaf: true);
}

class _ImportedStructFfi extends ffi.Struct {
  @ffi.Uint32()
  external int foo;
  @ffi.Uint8()
  external int count;
}

class ImportedStruct {
  final _ImportedStructFfi _underlying;

  // ignore: unused_element
  ImportedStruct._(this._underlying);

  factory ImportedStruct() {
    final pointer = ffi2.calloc<_ImportedStructFfi>();
    final result = ImportedStruct._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  UnimportedEnum get foo => UnimportedEnum._(_underlying.foo);
  set foo(UnimportedEnum foo) {
    _underlying.foo = foo._id;
  }

  int get count => _underlying.count;
  set count(int count) {
    _underlying.count = count;
  }

  @override
  bool operator ==(Object other) =>
      other is ImportedStruct &&
      other._underlying.foo == _underlying.foo &&
      other._underlying.count == _underlying.count;

  @override
  int get hashCode => Object.hashAll([
        _underlying.foo,
        _underlying.count,
      ]);
}

enum MyEnum {
  a.__(-2),
  b.__(-1),
  c.__(0),
  d.__(1),
  e.__(2),
  f.__(3);

  const MyEnum.__(this._id);

  // ignore: unused_element
  factory MyEnum._(int id) => values.firstWhere((value) => value._id == id);

  final int _id;

  int intoValue() {
    final result = _intoValueFfi(_id);
    return result;
  }

  static final _intoValueFfi =
      _capi<ffi.NativeFunction<ffi.Int8 Function(ffi.Uint32)>>(
              'MyEnum_into_value')
          .asFunction<int Function(int)>(isLeaf: true);
}

class MyString implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  MyString._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('MyString_destroy'));

  factory MyString(String v) {
    final alloc = ffi2.Arena();
    final vSlice = _SliceFfi2Utf8._fromDart(v, alloc);

    final result = _newFfi(vSlice._bytes, vSlice._length);
    alloc.releaseAll();
    return MyString._(result);
  }
  static final _newFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi2.Utf8>, ffi.Size)>>('MyString_new')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  void setStr(String newStr) {
    final alloc = ffi2.Arena();
    final newStrSlice = _SliceFfi2Utf8._fromDart(newStr, alloc);

    _setStrFfi(_underlying, newStrSlice._bytes, newStrSlice._length);
    alloc.releaseAll();
  }

  static final _setStrFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('MyString_set_str')
      .asFunction<
          void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);

  String get getStr {
    final writeable = _Writeable();
    _getStrFfi(_underlying, writeable._underlying);
    return writeable.finalize();
  }

  static final _getStrFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('MyString_get_str')
      .asFunction<
          void Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

class _MyStructFfi extends ffi.Struct {
  @ffi.Uint8()
  external int a;
  @ffi.Bool()
  external bool b;
  @ffi.Uint8()
  external int c;
  @ffi.Uint64()
  external int d;
  @ffi.Int32()
  external int e;
  @ffi.Uint32()
  external int f;
  @ffi.Uint32()
  external int g;
}

class MyStruct {
  final _MyStructFfi _underlying;

  // ignore: unused_element
  MyStruct._(this._underlying);

  factory MyStruct() {
    final pointer = ffi2.calloc<_MyStructFfi>();
    final result = MyStruct._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  int get a => _underlying.a;
  set a(int a) {
    _underlying.a = a;
  }

  bool get b => _underlying.b;
  set b(bool b) {
    _underlying.b = b;
  }

  int get c => _underlying.c;
  set c(int c) {
    _underlying.c = c;
  }

  int get d => _underlying.d;
  set d(int d) {
    _underlying.d = d;
  }

  int get e => _underlying.e;
  set e(int e) {
    _underlying.e = e;
  }

  int get f => _underlying.f;
  set f(int f) {
    _underlying.f = f;
  }

  MyEnum get g => MyEnum._(_underlying.g);
  set g(MyEnum g) {
    _underlying.g = g._id;
  }

  factory MyStruct() {
    final result = _newFfi();
    return MyStruct._(result);
  }
  static final _newFfi =
      _capi<ffi.NativeFunction<_MyStructFfi Function()>>('MyStruct_new')
          .asFunction<_MyStructFfi Function()>(isLeaf: true);

  int intoA() {
    final result = _intoAFfi(_underlying);
    return result;
  }

  static final _intoAFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(_MyStructFfi)>>(
              'MyStruct_into_a')
          .asFunction<int Function(_MyStructFfi)>(isLeaf: true);

  @override
  bool operator ==(Object other) =>
      other is MyStruct &&
      other._underlying.a == _underlying.a &&
      other._underlying.b == _underlying.b &&
      other._underlying.c == _underlying.c &&
      other._underlying.d == _underlying.d &&
      other._underlying.e == _underlying.e &&
      other._underlying.f == _underlying.f &&
      other._underlying.g == _underlying.g;

  @override
  int get hashCode => Object.hashAll([
        _underlying.a,
        _underlying.b,
        _underlying.c,
        _underlying.d,
        _underlying.e,
        _underlying.f,
        _underlying.g,
      ]);
}

class One implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  One._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('One_destroy'));

  factory One.transitivity(One hold, One nohold) {
    final result = _transitivityFfi(hold._underlying, nohold._underlying);
    return One._(result);
  }
  static final _transitivityFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_transitivity')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.cycle(Two hold, One nohold) {
    final result = _cycleFfi(hold._underlying, nohold._underlying);
    return One._(result);
  }
  static final _cycleFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_cycle')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.manyDependents(One a, One b, Two c, Two d, Two nohold) {
    final result = _manyDependentsFfi(a._underlying, b._underlying,
        c._underlying, d._underlying, nohold._underlying);
    return One._(result);
  }
  static final _manyDependentsFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_many_dependents')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.returnOutlivesParam(Two hold, One nohold) {
    final result =
        _returnOutlivesParamFfi(hold._underlying, nohold._underlying);
    return One._(result);
  }
  static final _returnOutlivesParamFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_return_outlives_param')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.diamondTop(One top, One left, One right, One bottom) {
    final result = _diamondTopFfi(top._underlying, left._underlying,
        right._underlying, bottom._underlying);
    return One._(result);
  }
  static final _diamondTopFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_diamond_top')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.diamondLeft(One top, One left, One right, One bottom) {
    final result = _diamondLeftFfi(top._underlying, left._underlying,
        right._underlying, bottom._underlying);
    return One._(result);
  }
  static final _diamondLeftFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_diamond_left')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.diamondRight(One top, One left, One right, One bottom) {
    final result = _diamondRightFfi(top._underlying, left._underlying,
        right._underlying, bottom._underlying);
    return One._(result);
  }
  static final _diamondRightFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_diamond_right')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.diamondBottom(One top, One left, One right, One bottom) {
    final result = _diamondBottomFfi(top._underlying, left._underlying,
        right._underlying, bottom._underlying);
    return One._(result);
  }
  static final _diamondBottomFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_diamond_bottom')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.diamondAndNestedTypes(One a, One b, One c, One d, One nohold) {
    final result = _diamondAndNestedTypesFfi(a._underlying, b._underlying,
        c._underlying, d._underlying, nohold._underlying);
    return One._(result);
  }
  static final _diamondAndNestedTypesFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_diamond_and_nested_types')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.implicitBounds(One explicitHold, One implicitHold, One nohold) {
    final result = _implicitBoundsFfi(
        explicitHold._underlying, implicitHold._underlying, nohold._underlying);
    return One._(result);
  }
  static final _implicitBoundsFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_implicit_bounds')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.implicitBoundsDeep(
      One explicit, One implicit1, One implicit2, One nohold) {
    final result = _implicitBoundsDeepFfi(explicit._underlying,
        implicit1._underlying, implicit2._underlying, nohold._underlying);
    return One._(result);
  }
  static final _implicitBoundsDeepFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('One_implicit_bounds_deep')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

class Opaque implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Opaque._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('Opaque_destroy'));

  factory Opaque() {
    final result = _newFfi();
    return Opaque._(result);
  }
  static final _newFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'Opaque_new')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// See the [Rust documentation for `something`](https://docs.rs/Something/latest/struct.Something.html#method.something) for more information.
  ///
  /// See the [Rust documentation for `something_else`](https://docs.rs/Something/latest/struct.Something.html#method.something_else) for more information.
  ///
  /// Additional information: [1](https://docs.rs/Something/latest/struct.Something.html#method.something_small), [2](https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something)
  void assertStruct(MyStruct s) {
    _assertStructFfi(_underlying, s._underlying);
  }

  static final _assertStructFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  _MyStructFfi)>>('Opaque_assert_struct')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, _MyStructFfi)>(
          isLeaf: true);

  static final int returnsUsize =
      _capi<ffi.NativeFunction<ffi.Size Function()>>('Opaque_returns_usize')
          .asFunction<int Function()>(isLeaf: true)();

  static final ImportedStruct returnsImported =
      _capi<ffi.NativeFunction<_ImportedStructFfi Function()>>(
              'Opaque_returns_imported')
          .asFunction<_ImportedStructFfi Function()>(isLeaf: true)();
}

class OptionOpaque implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  OptionOpaque._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('OptionOpaque_destroy'));

  static OptionOpaque? new_(int i) {
    final result = _newFfi(i);
    return result.address == 0 ? null : OptionOpaque._(result);
  }

  static final _newFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Int32)>>(
              'OptionOpaque_new')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);

  static final OptionOpaque? newNone =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'OptionOpaque_new_none')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true)();

  static final OptionStruct newStruct =
      _capi<ffi.NativeFunction<_OptionStructFfi Function()>>(
              'OptionOpaque_new_struct')
          .asFunction<_OptionStructFfi Function()>(isLeaf: true)();

  static final OptionStruct newStructNones =
      _capi<ffi.NativeFunction<_OptionStructFfi Function()>>(
              'OptionOpaque_new_struct_nones')
          .asFunction<_OptionStructFfi Function()>(isLeaf: true)();

  void assertInteger(int i) {
    _assertIntegerFfi(_underlying, i);
  }

  static final _assertIntegerFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int32)>>('OptionOpaque_assert_integer')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  static bool optionOpaqueArgument(OptionOpaque? arg) {
    final result = _optionOpaqueArgumentFfi(arg._underlying);
    return result;
  }

  static final _optionOpaqueArgumentFfi =
      _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>>(
              'OptionOpaque_option_opaque_argument')
          .asFunction<bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

class OptionOpaqueChar implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  OptionOpaqueChar._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('OptionOpaqueChar_destroy'));

  void assertChar(int ch) {
    _assertCharFfi(_underlying, ch);
  }

  static final _assertCharFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('OptionOpaqueChar_assert_char')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);
}

class _OptionStructFfi extends ffi.Struct {
  external ffi.Pointer<ffi.Opaque> a;
  external ffi.Pointer<ffi.Opaque> b;
  @ffi.Uint32()
  external int c;
  external ffi.Pointer<ffi.Opaque> d;
}

class OptionStruct {
  final _OptionStructFfi _underlying;

  // ignore: unused_element
  OptionStruct._(this._underlying);

  factory OptionStruct() {
    final pointer = ffi2.calloc<_OptionStructFfi>();
    final result = OptionStruct._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  OptionOpaque? get a =>
      _underlying.a.address == 0 ? null : OptionOpaque._(_underlying.a);
  set a(OptionOpaque? a) {
    _underlying.a = a._underlying;
  }

  OptionOpaqueChar? get b =>
      _underlying.b.address == 0 ? null : OptionOpaqueChar._(_underlying.b);
  set b(OptionOpaqueChar? b) {
    _underlying.b = b._underlying;
  }

  int get c => _underlying.c;
  set c(int c) {
    _underlying.c = c;
  }

  OptionOpaque? get d =>
      _underlying.d.address == 0 ? null : OptionOpaque._(_underlying.d);
  set d(OptionOpaque? d) {
    _underlying.d = d._underlying;
  }

  @override
  bool operator ==(Object other) =>
      other is OptionStruct &&
      other._underlying.a == _underlying.a &&
      other._underlying.b == _underlying.b &&
      other._underlying.c == _underlying.c &&
      other._underlying.d == _underlying.d;

  @override
  int get hashCode => Object.hashAll([
        _underlying.a,
        _underlying.b,
        _underlying.c,
        _underlying.d,
      ]);
}

class RefList implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  RefList._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('RefList_destroy'));

  factory RefList.node(RefListParameter data) {
    final result = _nodeFfi(data._underlying);
    return RefList._(result);
  }
  static final _nodeFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('RefList_node')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

class RefListParameter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  RefListParameter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('RefListParameter_destroy'));
}

class ResultOpaque implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ResultOpaque._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ResultOpaque_destroy'));

  factory ResultOpaque(int i) {
    final result = _newFfi(i);
    return result.isOk
        ? ResultOpaque._(result.union.ok)
        : throw ErrorEnum._(result.union.err);
  }
  static final _newFfi =
      _capi<ffi.NativeFunction<_ResultOpaqueUint32 Function(ffi.Int32)>>(
              'ResultOpaque_new')
          .asFunction<_ResultOpaqueUint32 Function(int)>(isLeaf: true);

  factory ResultOpaque.failingFoo() {
    final result = _newFailingFooFfi();
    return result.isOk
        ? ResultOpaque._(result.union.ok)
        : throw ErrorEnum._(result.union.err);
  }
  static final _newFailingFooFfi =
      _capi<ffi.NativeFunction<_ResultOpaqueUint32 Function()>>(
              'ResultOpaque_new_failing_foo')
          .asFunction<_ResultOpaqueUint32 Function()>(isLeaf: true);

  factory ResultOpaque.failingBar() {
    final result = _newFailingBarFfi();
    return result.isOk
        ? ResultOpaque._(result.union.ok)
        : throw ErrorEnum._(result.union.err);
  }
  static final _newFailingBarFfi =
      _capi<ffi.NativeFunction<_ResultOpaqueUint32 Function()>>(
              'ResultOpaque_new_failing_bar')
          .asFunction<_ResultOpaqueUint32 Function()>(isLeaf: true);

  factory ResultOpaque.failingUnit() {
    final result = _newFailingUnitFfi();
    return result.isOk ? ResultOpaque._(result.union.ok) : throw VoidError();
  }
  static final _newFailingUnitFfi =
      _capi<ffi.NativeFunction<_ResultOpaqueVoid Function()>>(
              'ResultOpaque_new_failing_unit')
          .asFunction<_ResultOpaqueVoid Function()>(isLeaf: true);

  factory ResultOpaque.failingStruct(int i) {
    final result = _newFailingStructFfi(i);
    return result.isOk
        ? ResultOpaque._(result.union.ok)
        : throw ErrorStruct._(result.union.err);
  }
  static final _newFailingStructFfi = _capi<
              ffi
              .NativeFunction<_ResultOpaqueErrorStructFfi Function(ffi.Int32)>>(
          'ResultOpaque_new_failing_struct')
      .asFunction<_ResultOpaqueErrorStructFfi Function(int)>(isLeaf: true);

  static void newInErr(int i) {
    final result = _newInErrFfi(i);
    if (!result.isOk) {
      throw ResultOpaque._(result.union.err);
    }
  }

  static final _newInErrFfi =
      _capi<ffi.NativeFunction<_ResultVoidOpaque Function(ffi.Int32)>>(
              'ResultOpaque_new_in_err')
          .asFunction<_ResultVoidOpaque Function(int)>(isLeaf: true);

  static int newInt(int i) {
    final result = _newIntFfi(i);
    return result.isOk ? result.union.ok : throw VoidError();
  }

  static final _newIntFfi =
      _capi<ffi.NativeFunction<_ResultInt32Void Function(ffi.Int32)>>(
              'ResultOpaque_new_int')
          .asFunction<_ResultInt32Void Function(int)>(isLeaf: true);

  static ErrorEnum newInEnumErr(int i) {
    final result = _newInEnumErrFfi(i);
    return result.isOk
        ? ErrorEnum._(result.union.ok)
        : throw ResultOpaque._(result.union.err);
  }

  static final _newInEnumErrFfi =
      _capi<ffi.NativeFunction<_ResultUint32Opaque Function(ffi.Int32)>>(
              'ResultOpaque_new_in_enum_err')
          .asFunction<_ResultUint32Opaque Function(int)>(isLeaf: true);

  void assertInteger(int i) {
    _assertIntegerFfi(_underlying, i);
  }

  static final _assertIntegerFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int32)>>('ResultOpaque_assert_integer')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);
}

class Two implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Two._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('Two_destroy'));
}

enum UnimportedEnum {
  a.__(0),
  b.__(1),
  c.__(2);

  const UnimportedEnum.__(this._id);

  // ignore: unused_element
  factory UnimportedEnum._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

class _ResultInt32VoidUnion extends ffi.Union {
  @ffi.Int32()
  external int ok;
}

class _ResultInt32Void extends ffi.Struct {
  external _ResultInt32VoidUnion union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultOpaqueErrorStructFfiUnion extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> ok;

  external _ErrorStructFfi err;
}

class _ResultOpaqueErrorStructFfi extends ffi.Struct {
  external _ResultOpaqueErrorStructFfiUnion union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultOpaqueUint32Union extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> ok;

  @ffi.Uint32()
  external int err;
}

class _ResultOpaqueUint32 extends ffi.Struct {
  external _ResultOpaqueUint32Union union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultOpaqueVoidUnion extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> ok;
}

class _ResultOpaqueVoid extends ffi.Struct {
  external _ResultOpaqueVoidUnion union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultUint32OpaqueUnion extends ffi.Union {
  @ffi.Uint32()
  external int ok;

  external ffi.Pointer<ffi.Opaque> err;
}

class _ResultUint32Opaque extends ffi.Struct {
  external _ResultUint32OpaqueUnion union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultVoidOpaqueUnion extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> err;
}

class _ResultVoidOpaque extends ffi.Struct {
  external _ResultVoidOpaqueUnion union;

  @ffi.Bool()
  external bool isOk;
}

class _SliceFfi2Utf8 extends ffi.Struct {
  external ffi.Pointer<ffi2.Utf8> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfi2Utf8 _fromDart(String value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfi2Utf8>();
    final slice = pointer.ref;
    final units = Utf8Encoder().convert(value);
    slice._length = units.length;
    slice._bytes = allocator<ffi.Uint8>(slice._length).cast();
    slice._bytes.cast<ffi.Uint8>().asTypedList(slice._length).setAll(0, units);

    return slice;
  }

  // ignore: unused_element
  String get _asDart =>
      Utf8Decoder().convert(_bytes.cast<ffi.Uint8>().asTypedList(_length));

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfi2Utf8 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes.cast<ffi.Uint8>()[i] != _bytes.cast<ffi.Uint8>()[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

class _SliceFfiDouble extends ffi.Struct {
  external ffi.Pointer<ffi.Double> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfiDouble _fromDart(Float64List value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfiDouble>();
    final slice = pointer.ref;
    slice._length = value.length;
    slice._bytes = allocator(slice._length);
    slice._bytes.asTypedList(slice._length).setAll(0, value);

    return slice;
  }

  // ignore: unused_element
  Float64List get _asDart => _bytes.asTypedList(_length);

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfiDouble || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

class _SliceFfiUint16 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint16> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfiUint16 _fromDart(Uint16List value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfiUint16>();
    final slice = pointer.ref;
    slice._length = value.length;
    slice._bytes = allocator(slice._length);
    slice._bytes.asTypedList(slice._length).setAll(0, value);

    return slice;
  }

  // ignore: unused_element
  Uint16List get _asDart => _bytes.asTypedList(_length);

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfiUint16 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

class _SliceFfiUint8 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfiUint8 _fromDart(Uint8List value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfiUint8>();
    final slice = pointer.ref;
    slice._length = value.length;
    slice._bytes = allocator(slice._length);
    slice._bytes.asTypedList(slice._length).setAll(0, value);

    return slice;
  }

  // ignore: unused_element
  Uint8List get _asDart => _bytes.asTypedList(_length);

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfiUint8 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

/// An unspecified error value
class VoidError {}

class _Writeable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  _Writeable() : _underlying = _create(0);
  static final _create =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>>(
              'diplomat_buffer_writeable_create')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>();

  String finalize() {
    final string =
        _getBytes(_underlying).toDartString(length: _len(_underlying));
    _destroy(_underlying);
    return string;
  }

  static final _len =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'diplomat_buffer_writeable_len')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
  static final _getBytes = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi2.Utf8> Function(ffi.Pointer<ffi.Opaque>)>>(
          'diplomat_buffer_writeable_get_bytes')
      .asFunction<ffi.Pointer<ffi2.Utf8> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
  static final _destroy =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'diplomat_buffer_writeable_destroy')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
