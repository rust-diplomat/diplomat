import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:typed_data';
import 'package:ffi/ffi.dart' as allocators;

late final ffi.Pointer<T> Function<T extends ffi.NativeType>(String) _capi;
void init(String path) => _capi = ffi.DynamicLibrary.open(path).lookup;



enum AttrEnum {
	A.__(0),
	B.__(1),
	C.__(2);

  const AttrEnum.__(this._id);

  factory AttrEnum._(int id) => values.firstWhere((value) => value._id == id);

  final int _id;

}


class AttrOpaque1 implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	AttrOpaque1._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('AttrOpaque1_destroy'));

	
void method() { 
		_methodFfi(this._underlying);
  }
	static late final _methodFfi = _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>('AttrOpaque1_method')
	.asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
void methodDisabledcpp() { 
		_methodDisabledcppFfi(this._underlying);
  }
	static late final _methodDisabledcppFfi = _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>('AttrOpaque1_method_disabledcpp')
	.asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	}


class AttrOpaque2 implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	AttrOpaque2._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('AttrOpaque2_destroy'));

	}


class Bar implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	Bar._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('Bar_destroy'));

	}


class _BorrowedFieldsFfi extends ffi.Struct {
		external _Slice a;
		external _Slice b;
}

class BorrowedFields {
	final _BorrowedFieldsFfi _underlying;

	BorrowedFields._(this._underlying);

 	factory BorrowedFields() {
		final pointer = allocators.calloc<_BorrowedFieldsFfi>();
		final result = BorrowedFields._(pointer.ref);
		_finalizer.attach(result, pointer.cast());
		return result;
	}
	static late final _finalizer = Finalizer(allocators.calloc.free);

	Uint16List get a => this._underlying.a.bytes.cast<ffi.Uint16>().asTypedList(this._underlying.a.length);
	void set a(Uint16List a) {
		final alloc = allocators.calloc;
		alloc.free(this._underlying.a.bytes);final aBytes = alloc.call<ffi.Uint16>(a.length);
aBytes.asTypedList(a.length).setAll(0, a);
this._underlying.a.bytes = aBytes.cast();this._underlying.a.length = a.length;
	}
String get b => Utf8Decoder(allowMalformed: false).convert(this._underlying.b.bytes.cast<ffi.Uint8>().asTypedList(this._underlying.b.length));
	void set b(String b) {
		final alloc = allocators.calloc;
		alloc.free(this._underlying.b.bytes);final bList = Utf8Encoder().convert(b);
		final bBytes = alloc.call<ffi.Char>(bList.length);
		bBytes.cast<ffi.Uint8>().asTypedList(bList.length).setAll(0, bList);
this._underlying.b.bytes = bBytes.cast();this._underlying.b.length = bList.length;
	}

}


class _BorrowedFieldsReturningFfi extends ffi.Struct {
		external _Slice bytes;
}

class BorrowedFieldsReturning {
	final _BorrowedFieldsReturningFfi _underlying;

	BorrowedFieldsReturning._(this._underlying);

 	factory BorrowedFieldsReturning() {
		final pointer = allocators.calloc<_BorrowedFieldsReturningFfi>();
		final result = BorrowedFieldsReturning._(pointer.ref);
		_finalizer.attach(result, pointer.cast());
		return result;
	}
	static late final _finalizer = Finalizer(allocators.calloc.free);

	Uint8List get bytes => this._underlying.bytes.bytes.cast<ffi.Uint8>().asTypedList(this._underlying.bytes.length);
	void set bytes(Uint8List bytes) {
		final alloc = allocators.calloc;
		alloc.free(this._underlying.bytes.bytes);final bytesBytes = alloc.call<ffi.Uint8>(bytes.length);
bytesBytes.asTypedList(bytes.length).setAll(0, bytes);
this._underlying.bytes.bytes = bytesBytes.cast();this._underlying.bytes.length = bytes.length;
	}

}


enum ErrorEnum {
	Foo.__(0),
	Bar.__(1);

  const ErrorEnum.__(this._id);

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

	ErrorStruct._(this._underlying);

 	factory ErrorStruct() {
		final pointer = allocators.calloc<_ErrorStructFfi>();
		final result = ErrorStruct._(pointer.ref);
		_finalizer.attach(result, pointer.cast());
		return result;
	}
	static late final _finalizer = Finalizer(allocators.calloc.free);

	int get i => this._underlying.i;
	void set i(int i) {this._underlying.i = i;
	}
int get j => this._underlying.j;
	void set j(int j) {this._underlying.j = j;
	}

}


class Float64Vec implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	Float64Vec._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('Float64Vec_destroy'));

	
factory Float64Vec(Float64List v) { 
		final alloc = allocators.Arena();
		
		final vBytes = alloc.call<ffi.Double>(v.length);
vBytes.asTypedList(v.length).setAll(0, v);

		final result = _newFfi(vBytes.cast(),v.length);alloc.releaseAll();
		return Float64Vec._(result);
  }
	static late final _newFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Double>, ffi.Size)>>('Float64Vec_new')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Double>, int)>(isLeaf: true);


	
void fillSlice(Float64List v) { 
		final alloc = allocators.Arena();
		
		final vBytes = alloc.call<ffi.Double>(v.length);
vBytes.asTypedList(v.length).setAll(0, v);

		_fillSliceFfi(this._underlying,vBytes.cast(),v.length);alloc.releaseAll();
  }
	static late final _fillSliceFfi = _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Double>, ffi.Size)>>('Float64Vec_fill_slice')
	.asFunction<void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Double>, int)>(isLeaf: true);


	
void setValue(Float64List newSlice) { 
		final alloc = allocators.Arena();
		
		final newSliceBytes = alloc.call<ffi.Double>(newSlice.length);
newSliceBytes.asTypedList(newSlice.length).setAll(0, newSlice);

		_setValueFfi(this._underlying,newSliceBytes.cast(),newSlice.length);alloc.releaseAll();
  }
	static late final _setValueFfi = _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Double>, ffi.Size)>>('Float64Vec_set_value')
	.asFunction<void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Double>, int)>(isLeaf: true);


	}


class Foo implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	Foo._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('Foo_destroy'));

	
factory Foo(String x) { 
		final alloc = allocators.Arena();
		
		final xList = Utf8Encoder().convert(x);
		final xBytes = alloc.call<ffi.Char>(xList.length);
		xBytes.cast<ffi.Uint8>().asTypedList(xList.length).setAll(0, xList);

		final result = _newFfi(xBytes.cast(),xList.length);alloc.releaseAll();
		return Foo._(result);
  }
	static late final _newFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Char>, ffi.Size)>>('Foo_new')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Char>, int)>(isLeaf: true);


	
Bar get getBar { 
		final result = _getBarFfi(this._underlying);
		return Bar._(result);
  }
	static late final _getBarFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>>('Foo_get_bar')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory Foo.static(String x) { 
		final alloc = allocators.Arena();
		
		final xList = Utf8Encoder().convert(x);
		final xBytes = alloc.call<ffi.Char>(xList.length);
		xBytes.cast<ffi.Uint8>().asTypedList(xList.length).setAll(0, xList);

		final result = _newStaticFfi(xBytes.cast(),xList.length);alloc.releaseAll();
		return Foo._(result);
  }
	static late final _newStaticFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Char>, ffi.Size)>>('Foo_new_static')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Char>, int)>(isLeaf: true);


	
BorrowedFieldsReturning get asReturning { 
		final result = _asReturningFfi(this._underlying);
		return BorrowedFieldsReturning._(result);
  }
	static late final _asReturningFfi = _capi<ffi.NativeFunction<_BorrowedFieldsReturningFfi Function(ffi.Pointer<ffi.Opaque>)>>('Foo_as_returning')
	.asFunction<_BorrowedFieldsReturningFfi Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory Foo.extractFromFields(BorrowedFields fields) { 
		final result = _extractFromFieldsFfi(fields._underlying);
		return Foo._(result);
  }
	static late final _extractFromFieldsFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(_BorrowedFieldsFfi)>>('Foo_extract_from_fields')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(_BorrowedFieldsFfi)>(isLeaf: true);


	}


class _ImportedStructFfi extends ffi.Struct {
		@ffi.Int32()
		external int foo;
		@ffi.Uint8()
		external int count;
}

class ImportedStruct {
	final _ImportedStructFfi _underlying;

	ImportedStruct._(this._underlying);

 	factory ImportedStruct() {
		final pointer = allocators.calloc<_ImportedStructFfi>();
		final result = ImportedStruct._(pointer.ref);
		_finalizer.attach(result, pointer.cast());
		return result;
	}
	static late final _finalizer = Finalizer(allocators.calloc.free);

	UnimportedEnum get foo => UnimportedEnum._(this._underlying.foo);
	void set foo(UnimportedEnum foo) {this._underlying.foo = foo._id;
	}
int get count => this._underlying.count;
	void set count(int count) {this._underlying.count = count;
	}

}


enum MyEnum {
	A.__(-2),
	B.__(-1),
	C.__(0),
	D.__(1),
	E.__(2),
	F.__(3);

  const MyEnum.__(this._id);

  factory MyEnum._(int id) => values.firstWhere((value) => value._id == id);

  final int _id;


int intoValue() { 
		final result = _intoValueFfi(this._id);
		return result;
  }
	static late final _intoValueFfi = _capi<ffi.NativeFunction<ffi.Int8 Function(ffi.Uint32)>>('MyEnum_into_value')
	.asFunction<int Function(int)>(isLeaf: true);


}


class MyString implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	MyString._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('MyString_destroy'));

	
factory MyString(String v) { 
		final alloc = allocators.Arena();
		
		final vList = Utf8Encoder().convert(v);
		final vBytes = alloc.call<ffi.Char>(vList.length);
		vBytes.cast<ffi.Uint8>().asTypedList(vList.length).setAll(0, vList);

		final result = _newFfi(vBytes.cast(),vList.length);alloc.releaseAll();
		return MyString._(result);
  }
	static late final _newFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Char>, ffi.Size)>>('MyString_new')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Char>, int)>(isLeaf: true);


	
void setStr(String newStr) { 
		final alloc = allocators.Arena();
		
		final newStrList = Utf8Encoder().convert(newStr);
		final newStrBytes = alloc.call<ffi.Char>(newStrList.length);
		newStrBytes.cast<ffi.Uint8>().asTypedList(newStrList.length).setAll(0, newStrList);

		_setStrFfi(this._underlying,newStrBytes.cast(),newStrList.length);alloc.releaseAll();
  }
	static late final _setStrFfi = _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Char>, ffi.Size)>>('MyString_set_str')
	.asFunction<void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Char>, int)>(isLeaf: true);


	
String get getStr { 
		final writeable = _Writeable();
		_getStrFfi(this._underlying,writeable._underlying);
		return writeable.toString();
  }
	static late final _getStrFfi = _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('MyString_get_str')
	.asFunction<void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


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
		@ffi.Int32()
		external int g;
}

class MyStruct {
	final _MyStructFfi _underlying;

	MyStruct._(this._underlying);

 	factory MyStruct() {
		final pointer = allocators.calloc<_MyStructFfi>();
		final result = MyStruct._(pointer.ref);
		_finalizer.attach(result, pointer.cast());
		return result;
	}
	static late final _finalizer = Finalizer(allocators.calloc.free);

	int get a => this._underlying.a;
	void set a(int a) {this._underlying.a = a;
	}
bool get b => this._underlying.b;
	void set b(bool b) {this._underlying.b = b;
	}
int get c => this._underlying.c;
	void set c(int c) {this._underlying.c = c;
	}
int get d => this._underlying.d;
	void set d(int d) {this._underlying.d = d;
	}
int get e => this._underlying.e;
	void set e(int e) {this._underlying.e = e;
	}
int get f => this._underlying.f;
	void set f(int f) {this._underlying.f = f;
	}
MyEnum get g => MyEnum._(this._underlying.g);
	void set g(MyEnum g) {this._underlying.g = g._id;
	}

factory MyStruct() { 
		final result = _newFfi();
		return MyStruct._(result);
  }
	static late final _newFfi = _capi<ffi.NativeFunction<_MyStructFfi Function()>>('MyStruct_new')
	.asFunction<_MyStructFfi Function()>(isLeaf: true);

int intoA() { 
		final result = _intoAFfi(this._underlying);
		return result;
  }
	static late final _intoAFfi = _capi<ffi.NativeFunction<ffi.Uint8 Function(_MyStructFfi)>>('MyStruct_into_a')
	.asFunction<int Function(_MyStructFfi)>(isLeaf: true);

}


class One implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	One._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('One_destroy'));

	
factory One.transitivity(One hold, One nohold) { 
		final result = _transitivityFfi(hold._underlying,nohold._underlying);
		return One._(result);
  }
	static late final _transitivityFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_transitivity')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory One.cycle(Two hold, One nohold) { 
		final result = _cycleFfi(hold._underlying,nohold._underlying);
		return One._(result);
  }
	static late final _cycleFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_cycle')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory One.manyDependents(One a, One b, Two c, Two d, Two nohold) { 
		final result = _manyDependentsFfi(a._underlying,b._underlying,c._underlying,d._underlying,nohold._underlying);
		return One._(result);
  }
	static late final _manyDependentsFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_many_dependents')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory One.returnOutlivesParam(Two hold, One nohold) { 
		final result = _returnOutlivesParamFfi(hold._underlying,nohold._underlying);
		return One._(result);
  }
	static late final _returnOutlivesParamFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_return_outlives_param')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory One.diamondTop(One top, One left, One right, One bottom) { 
		final result = _diamondTopFfi(top._underlying,left._underlying,right._underlying,bottom._underlying);
		return One._(result);
  }
	static late final _diamondTopFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_diamond_top')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory One.diamondLeft(One top, One left, One right, One bottom) { 
		final result = _diamondLeftFfi(top._underlying,left._underlying,right._underlying,bottom._underlying);
		return One._(result);
  }
	static late final _diamondLeftFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_diamond_left')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory One.diamondRight(One top, One left, One right, One bottom) { 
		final result = _diamondRightFfi(top._underlying,left._underlying,right._underlying,bottom._underlying);
		return One._(result);
  }
	static late final _diamondRightFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_diamond_right')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory One.diamondBottom(One top, One left, One right, One bottom) { 
		final result = _diamondBottomFfi(top._underlying,left._underlying,right._underlying,bottom._underlying);
		return One._(result);
  }
	static late final _diamondBottomFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_diamond_bottom')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory One.diamondAndNestedTypes(One a, One b, One c, One d, One nohold) { 
		final result = _diamondAndNestedTypesFfi(a._underlying,b._underlying,c._underlying,d._underlying,nohold._underlying);
		return One._(result);
  }
	static late final _diamondAndNestedTypesFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_diamond_and_nested_types')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory One.implicitBounds(One explicitHold, One implicitHold, One nohold) { 
		final result = _implicitBoundsFfi(explicitHold._underlying,implicitHold._underlying,nohold._underlying);
		return One._(result);
  }
	static late final _implicitBoundsFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_implicit_bounds')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	
factory One.implicitBoundsDeep(One explicit, One implicit1, One implicit2, One nohold) { 
		final result = _implicitBoundsDeepFfi(explicit._underlying,implicit1._underlying,implicit2._underlying,nohold._underlying);
		return One._(result);
  }
	static late final _implicitBoundsDeepFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_implicit_bounds_deep')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	}


class Opaque implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	Opaque._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('Opaque_destroy'));

	
factory Opaque() { 
		final result = _newFfi();
		return Opaque._(result);
  }
	static late final _newFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>('Opaque_new')
	.asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);


	
/// See the [Rust documentation for `something`](https://docs.rs/Something/latest/struct.Something.html#method.something) for more information.
/// 
/// See the [Rust documentation for `something_else`](https://docs.rs/Something/latest/struct.Something.html#method.something_else) for more information.
/// 
/// Additional information: [1](https://docs.rs/Something/latest/struct.Something.html#method.something_small), [2](https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something)
void assertStruct(MyStruct s) { 
		_assertStructFfi(this._underlying,s._underlying);
  }
	static late final _assertStructFfi = _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, _MyStructFfi)>>('Opaque_assert_struct')
	.asFunction<void Function(ffi.Pointer<ffi.Opaque>, _MyStructFfi)>(isLeaf: true);


	
static late final int returnsUsize = _capi<ffi.NativeFunction<ffi.Uint64 Function()>>('Opaque_returns_usize').asFunction<int Function()>(isLeaf: true)();

	
static late final ImportedStruct returnsImported = _capi<ffi.NativeFunction<_ImportedStructFfi Function()>>('Opaque_returns_imported').asFunction<_ImportedStructFfi Function()>(isLeaf: true)();

	}


class OptionOpaque implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	OptionOpaque._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('OptionOpaque_destroy'));

	
static OptionOpaque? new(int i) { 
		final result = _newFfi(i);
		return result.address == 0 ? null : OptionOpaque._(result);
  }
	static late final _newFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Int32)>>('OptionOpaque_new')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);


	
static late final OptionOpaque? newNone = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>('OptionOpaque_new_none').asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true)();

	
static late final OptionStruct newStruct = _capi<ffi.NativeFunction<_OptionStructFfi Function()>>('OptionOpaque_new_struct').asFunction<_OptionStructFfi Function()>(isLeaf: true)();

	
static late final OptionStruct newStructNones = _capi<ffi.NativeFunction<_OptionStructFfi Function()>>('OptionOpaque_new_struct_nones').asFunction<_OptionStructFfi Function()>(isLeaf: true)();

	
void assertInteger(int i) { 
		_assertIntegerFfi(this._underlying,i);
  }
	static late final _assertIntegerFfi = _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Int32)>>('OptionOpaque_assert_integer')
	.asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);


	
static bool optionOpaqueArgument(OptionOpaque? arg) { 
		final result = _optionOpaqueArgumentFfi(arg._underlying);
		return result;
  }
	static late final _optionOpaqueArgumentFfi = _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>>('OptionOpaque_option_opaque_argument')
	.asFunction<bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	}


class OptionOpaqueChar implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	OptionOpaqueChar._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('OptionOpaqueChar_destroy'));

	
void assertChar(int ch) { 
		_assertCharFfi(this._underlying,ch);
  }
	static late final _assertCharFfi = _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32)>>('OptionOpaqueChar_assert_char')
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

	OptionStruct._(this._underlying);

 	factory OptionStruct() {
		final pointer = allocators.calloc<_OptionStructFfi>();
		final result = OptionStruct._(pointer.ref);
		_finalizer.attach(result, pointer.cast());
		return result;
	}
	static late final _finalizer = Finalizer(allocators.calloc.free);

	OptionOpaque? get a => this._underlying.a.address == 0 ? null : OptionOpaque._(this._underlying.a);
	void set a(OptionOpaque? a) {
		final alloc = allocators.calloc;
		alloc.free(this._underlying.a);this._underlying.a = a._underlying;
	}
OptionOpaqueChar? get b => this._underlying.b.address == 0 ? null : OptionOpaqueChar._(this._underlying.b);
	void set b(OptionOpaqueChar? b) {
		final alloc = allocators.calloc;
		alloc.free(this._underlying.b);this._underlying.b = b._underlying;
	}
int get c => this._underlying.c;
	void set c(int c) {this._underlying.c = c;
	}
OptionOpaque? get d => this._underlying.d.address == 0 ? null : OptionOpaque._(this._underlying.d);
	void set d(OptionOpaque? d) {
		final alloc = allocators.calloc;
		alloc.free(this._underlying.d);this._underlying.d = d._underlying;
	}

}


class RefList implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	RefList._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('RefList_destroy'));

	
factory RefList.node(RefListParameter data) { 
		final result = _nodeFfi(data._underlying);
		return RefList._(result);
  }
	static late final _nodeFfi = _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>>('RefList_node')
	.asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);


	}


class RefListParameter implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	RefListParameter._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('RefListParameter_destroy'));

	}


class ResultOpaque implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	ResultOpaque._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('ResultOpaque_destroy'));

	
factory ResultOpaque(int i) { 
		final result = _newFfi(i);
		return result.isOk ? ResultOpaque._(result.union.ok) : throw ErrorEnum._(result.union.err);
  }
	static late final _newFfi = _capi<ffi.NativeFunction<_ResultOpaqueUint32 Function(ffi.Int32)>>('ResultOpaque_new')
	.asFunction<_ResultOpaqueUint32 Function(int)>(isLeaf: true);


	
factory ResultOpaque.failingFoo() { 
		final result = _newFailingFooFfi();
		return result.isOk ? ResultOpaque._(result.union.ok) : throw ErrorEnum._(result.union.err);
  }
	static late final _newFailingFooFfi = _capi<ffi.NativeFunction<_ResultOpaqueUint32 Function()>>('ResultOpaque_new_failing_foo')
	.asFunction<_ResultOpaqueUint32 Function()>(isLeaf: true);


	
factory ResultOpaque.failingBar() { 
		final result = _newFailingBarFfi();
		return result.isOk ? ResultOpaque._(result.union.ok) : throw ErrorEnum._(result.union.err);
  }
	static late final _newFailingBarFfi = _capi<ffi.NativeFunction<_ResultOpaqueUint32 Function()>>('ResultOpaque_new_failing_bar')
	.asFunction<_ResultOpaqueUint32 Function()>(isLeaf: true);


	
factory ResultOpaque.failingUnit() { 
		final result = _newFailingUnitFfi();
		return result.isOk ? ResultOpaque._(result.union.ok) : throw VoidError();
  }
	static late final _newFailingUnitFfi = _capi<ffi.NativeFunction<_ResultOpaqueVoid Function()>>('ResultOpaque_new_failing_unit')
	.asFunction<_ResultOpaqueVoid Function()>(isLeaf: true);


	
factory ResultOpaque.failingStruct(int i) { 
		final result = _newFailingStructFfi(i);
		return result.isOk ? ResultOpaque._(result.union.ok) : throw ErrorStruct._(result.union.err);
  }
	static late final _newFailingStructFfi = _capi<ffi.NativeFunction<_ResultOpaque_ErrorStructFfi Function(ffi.Int32)>>('ResultOpaque_new_failing_struct')
	.asFunction<_ResultOpaque_ErrorStructFfi Function(int)>(isLeaf: true);


	
static void newInErr(int i) { 
		final result = _newInErrFfi(i);
		if (!result.isOk) { throw ResultOpaque._(result.union.err); }
  }
	static late final _newInErrFfi = _capi<ffi.NativeFunction<_ResultVoidOpaque Function(ffi.Int32)>>('ResultOpaque_new_in_err')
	.asFunction<_ResultVoidOpaque Function(int)>(isLeaf: true);


	
static int newInt(int i) { 
		final result = _newIntFfi(i);
		return result.isOk ? result.union.ok : throw VoidError();
  }
	static late final _newIntFfi = _capi<ffi.NativeFunction<_ResultInt32Void Function(ffi.Int32)>>('ResultOpaque_new_int')
	.asFunction<_ResultInt32Void Function(int)>(isLeaf: true);


	
static ErrorEnum newInEnumErr(int i) { 
		final result = _newInEnumErrFfi(i);
		return result.isOk ? ErrorEnum._(result.union.ok) : throw ResultOpaque._(result.union.err);
  }
	static late final _newInEnumErrFfi = _capi<ffi.NativeFunction<_ResultUint32Opaque Function(ffi.Int32)>>('ResultOpaque_new_in_enum_err')
	.asFunction<_ResultUint32Opaque Function(int)>(isLeaf: true);


	
void assertInteger(int i) { 
		_assertIntegerFfi(this._underlying,i);
  }
	static late final _assertIntegerFfi = _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Int32)>>('ResultOpaque_assert_integer')
	.asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);


	}


class Two implements ffi.Finalizable {

	final ffi.Pointer<ffi.Opaque> _underlying;

	Two._(this._underlying) {
		_finalizer.attach(this, this._underlying.cast());
	}

	static late final _finalizer = ffi.NativeFinalizer(_capi('Two_destroy'));

	}


enum UnimportedEnum {
	A.__(0),
	B.__(1),
	C.__(2);

  const UnimportedEnum.__(this._id);

  factory UnimportedEnum._(int id) => values.firstWhere((value) => value._id == id);

  final int _id;

}

class _UnionInt32Void extends ffi.Union {
    @ffi.Int32()
		external int ok;

}
class _ResultInt32Void extends ffi.Struct {
    external _UnionInt32Void union;

    @ffi.Bool()
    external bool isOk;
}

class _UnionOpaqueUint32 extends ffi.Union {
    external ffi.Pointer<ffi.Opaque> ok;

    @ffi.Int32()
		external int err;

}
class _ResultOpaqueUint32 extends ffi.Struct {
    external _UnionOpaqueUint32 union;

    @ffi.Bool()
    external bool isOk;
}

class _UnionOpaqueVoid extends ffi.Union {
    external ffi.Pointer<ffi.Opaque> ok;

}
class _ResultOpaqueVoid extends ffi.Struct {
    external _UnionOpaqueVoid union;

    @ffi.Bool()
    external bool isOk;
}

class _UnionOpaque_ErrorStructFfi extends ffi.Union {
    external ffi.Pointer<ffi.Opaque> ok;

    external _ErrorStructFfi err;

}
class _ResultOpaque_ErrorStructFfi extends ffi.Struct {
    external _UnionOpaque_ErrorStructFfi union;

    @ffi.Bool()
    external bool isOk;
}

class _UnionUint32Opaque extends ffi.Union {
    @ffi.Int32()
		external int ok;

    external ffi.Pointer<ffi.Opaque> err;

}
class _ResultUint32Opaque extends ffi.Struct {
    external _UnionUint32Opaque union;

    @ffi.Bool()
    external bool isOk;
}

class _UnionVoidOpaque extends ffi.Union {
    external ffi.Pointer<ffi.Opaque> err;

}
class _ResultVoidOpaque extends ffi.Struct {
    external _UnionVoidOpaque union;

    @ffi.Bool()
    external bool isOk;
}

class _Slice extends ffi.Struct {
  external ffi.Pointer<ffi.Void> bytes;

  @ffi.Size()
  external int length;
}


/// An unspecified error value
class VoidError {}

class _Writeable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  _Writeable() : _underlying = _create(0);
  static late final _create =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>>(
              "diplomat_buffer_writeable_create")
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>();

  String toString() {
    final string = Utf8Decoder(allowMalformed: false)
        .convert(_get_bytes(_underlying).cast<ffi.Uint8>().asTypedList(_len(_underlying)));
    _destroy(_underlying);
    return string;
  }
  static late final _len =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              "diplomat_buffer_writeable_len")
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
  static late final _get_bytes = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Opaque>)>>(
          "diplomat_buffer_writeable_get_bytes")
      .asFunction<ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
  static late final _destroy =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              "diplomat_buffer_writeable_destroy")
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}


