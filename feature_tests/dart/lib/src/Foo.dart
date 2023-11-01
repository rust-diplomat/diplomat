import 'Bar.dart';
import 'BorrowedFields.dart';
import 'BorrowedFieldsReturning.dart';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';
import 'package:ffi/ffi.dart' as allocators;

typedef FooFfi = ffi.Pointer<ffi.Opaque>;

class Foo implements ffi.Finalizable {
  final FooFfi _underlying;

  factory Foo.new(String x) {
    final alloc = allocators.Arena();

    final xList = Utf8Encoder().convert(x);
    final xBytes = alloc.call<ffi.Char>(xList.length);
    xBytes.cast<ffi.Uint8>().asTypedList(xList.length).setAll(0, xList);

    final result = _newFfi(xBytes.cast(), xList.length);
    alloc.releaseAll();
    return FooFromFfi(result);
  }
  static late final _newFfi = capi<
          ffi.NativeFunction<
              FooFfi Function(ffi.Pointer<ffi.Char>, ffi.Size)>>('Foo_new')
      .asFunction<FooFfi Function(ffi.Pointer<ffi.Char>, int)>();

  Bar getBar() {
    final result = _getBarFfi(this._underlying);
    return BarFromFfi(result);
  }

  static late final _getBarFfi =
      capi<ffi.NativeFunction<BarFfi Function(FooFfi)>>('Foo_get_bar')
          .asFunction<BarFfi Function(FooFfi)>();

  factory Foo.newStatic(String x) {
    final alloc = allocators.Arena();

    final xList = Utf8Encoder().convert(x);
    final xBytes = alloc.call<ffi.Char>(xList.length);
    xBytes.cast<ffi.Uint8>().asTypedList(xList.length).setAll(0, xList);

    final result = _newStaticFfi(xBytes.cast(), xList.length);
    alloc.releaseAll();
    return FooFromFfi(result);
  }
  static late final _newStaticFfi = capi<
          ffi.NativeFunction<
              FooFfi Function(
                  ffi.Pointer<ffi.Char>, ffi.Size)>>('Foo_new_static')
      .asFunction<FooFfi Function(ffi.Pointer<ffi.Char>, int)>();

  BorrowedFieldsReturning asReturning() {
    final result = _asReturningFfi(this._underlying);
    return BorrowedFieldsReturningFromFfi(result);
  }

  static late final _asReturningFfi =
      capi<ffi.NativeFunction<BorrowedFieldsReturningFfi Function(FooFfi)>>(
              'Foo_as_returning')
          .asFunction<BorrowedFieldsReturningFfi Function(FooFfi)>();

  factory Foo.extractFromFields(BorrowedFields fields) {
    final result = _extractFromFieldsFfi(BorrowedFieldsAsFfi(fields));
    return FooFromFfi(result);
  }
  static late final _extractFromFieldsFfi =
      capi<ffi.NativeFunction<FooFfi Function(BorrowedFieldsFfi)>>(
              'Foo_extract_from_fields')
          .asFunction<FooFfi Function(BorrowedFieldsFfi)>();

  Foo._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'Foo_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
Foo FooFromFfi(FooFfi underlying) => Foo._(underlying);
FooFfi FooAsFfi(Foo t) => t._underlying;
