import 'ErrorEnum.dart';
import 'ErrorStruct.dart';
import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef ResultOpaqueFfi = ffi.Pointer<ffi.Opaque>;

class ResultOpaque implements ffi.Finalizable {
  final ResultOpaqueFfi _underlying;

  factory ResultOpaque.new(int i) {
    final result = _newFfi(i);
    return result.isOk
        ? ResultOpaqueFromFfi(result.union.ok)
        : throw ErrorEnumFromFfi(result.union.err);
  }
  static late final _newFfi = capi<
          ffi.NativeFunction<
              ResultResultOpaqueUnionErrorEnum Function(
                  ffi.Int32)>>('ResultOpaque_new')
      .asFunction<ResultResultOpaqueUnionErrorEnum Function(int)>();

  factory ResultOpaque.newFailingFoo() {
    final result = _newFailingFooFfi();
    return result.isOk
        ? ResultOpaqueFromFfi(result.union.ok)
        : throw ErrorEnumFromFfi(result.union.err);
  }
  static late final _newFailingFooFfi =
      capi<ffi.NativeFunction<ResultResultOpaqueUnionErrorEnum Function()>>(
              'ResultOpaque_new_failing_foo')
          .asFunction<ResultResultOpaqueUnionErrorEnum Function()>();

  factory ResultOpaque.newFailingBar() {
    final result = _newFailingBarFfi();
    return result.isOk
        ? ResultOpaqueFromFfi(result.union.ok)
        : throw ErrorEnumFromFfi(result.union.err);
  }
  static late final _newFailingBarFfi =
      capi<ffi.NativeFunction<ResultResultOpaqueUnionErrorEnum Function()>>(
              'ResultOpaque_new_failing_bar')
          .asFunction<ResultResultOpaqueUnionErrorEnum Function()>();

  factory ResultOpaque.newFailingUnit() {
    final result = _newFailingUnitFfi();
    return result.isOk
        ? ResultOpaqueFromFfi(result.union.ok)
        : throw VoidError();
  }
  static late final _newFailingUnitFfi =
      capi<ffi.NativeFunction<ResultResultOpaqueUnionVoid Function()>>(
              'ResultOpaque_new_failing_unit')
          .asFunction<ResultResultOpaqueUnionVoid Function()>();

  factory ResultOpaque.newFailingStruct(int i) {
    final result = _newFailingStructFfi(i);
    return result.isOk
        ? ResultOpaqueFromFfi(result.union.ok)
        : throw ErrorStructFromFfi(result.union.err);
  }
  static late final _newFailingStructFfi = capi<
          ffi.NativeFunction<
              ResultResultOpaqueUnionErrorStruct Function(
                  ffi.Int32)>>('ResultOpaque_new_failing_struct')
      .asFunction<ResultResultOpaqueUnionErrorStruct Function(int)>();

  static void newInErr(int i) {
    final result = _newInErrFfi(i);
    if (!result.isOk) {
      throw ResultOpaqueFromFfi(result.union.err);
    }
  }

  static late final _newInErrFfi =
      capi<ffi.NativeFunction<ResultVoidUnionResultOpaque Function(ffi.Int32)>>(
              'ResultOpaque_new_in_err')
          .asFunction<ResultVoidUnionResultOpaque Function(int)>();

  static int newInt(int i) {
    final result = _newIntFfi(i);
    return result.isOk ? result.union.ok : throw VoidError();
  }

  static late final _newIntFfi =
      capi<ffi.NativeFunction<ResultintUnionVoid Function(ffi.Int32)>>(
              'ResultOpaque_new_int')
          .asFunction<ResultintUnionVoid Function(int)>();

  static ErrorEnum newInEnumErr(int i) {
    final result = _newInEnumErrFfi(i);
    return result.isOk
        ? ErrorEnumFromFfi(result.union.ok)
        : throw ResultOpaqueFromFfi(result.union.err);
  }

  static late final _newInEnumErrFfi = capi<
          ffi.NativeFunction<
              ResultErrorEnumUnionResultOpaque Function(
                  ffi.Int32)>>('ResultOpaque_new_in_enum_err')
      .asFunction<ResultErrorEnumUnionResultOpaque Function(int)>();

  void assertInteger(int i) {
    _assertIntegerFfi(this._underlying, i);
  }

  static late final _assertIntegerFfi =
      capi<ffi.NativeFunction<ffi.Void Function(ResultOpaqueFfi, ffi.Int32)>>(
              'ResultOpaque_assert_integer')
          .asFunction<void Function(ResultOpaqueFfi, int)>();

  ResultOpaque._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'ResultOpaque_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
ResultOpaque ResultOpaqueFromFfi(ResultOpaqueFfi underlying) =>
    ResultOpaque._(underlying);
ResultOpaqueFfi ResultOpaqueAsFfi(ResultOpaque t) => t._underlying;

class ErrorEnumUnionResultOpaque extends ffi.Union {
  @ffi.Int32()
  external ErrorEnumFfi ok;

  external ResultOpaqueFfi err;
}

class ResultErrorEnumUnionResultOpaque extends ffi.Struct {
  external ErrorEnumUnionResultOpaque union;

  @ffi.Bool()
  external bool isOk;
}

class ResultOpaqueUnionErrorEnum extends ffi.Union {
  external ResultOpaqueFfi ok;

  @ffi.Int32()
  external ErrorEnumFfi err;
}

class ResultResultOpaqueUnionErrorEnum extends ffi.Struct {
  external ResultOpaqueUnionErrorEnum union;

  @ffi.Bool()
  external bool isOk;
}

class ResultOpaqueUnionErrorStruct extends ffi.Union {
  external ResultOpaqueFfi ok;

  external ErrorStructFfi err;
}

class ResultResultOpaqueUnionErrorStruct extends ffi.Struct {
  external ResultOpaqueUnionErrorStruct union;

  @ffi.Bool()
  external bool isOk;
}

class ResultOpaqueUnionVoid extends ffi.Union {
  external ResultOpaqueFfi ok;
}

class ResultResultOpaqueUnionVoid extends ffi.Struct {
  external ResultOpaqueUnionVoid union;

  @ffi.Bool()
  external bool isOk;
}

class VoidUnionResultOpaque extends ffi.Union {
  external ResultOpaqueFfi err;
}

class ResultVoidUnionResultOpaque extends ffi.Struct {
  external VoidUnionResultOpaque union;

  @ffi.Bool()
  external bool isOk;
}

class intUnionVoid extends ffi.Union {
  @ffi.Int32()
  external int ok;
}

class ResultintUnionVoid extends ffi.Struct {
  external intUnionVoid union;

  @ffi.Bool()
  external bool isOk;
}
