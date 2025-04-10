// generated by diplomat-tool
// dart format off

part of 'lib.g.dart';

final class ResultOpaque implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  ResultOpaque._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ResultOpaque_destroy));

  ///
  ///
  /// Throws [ErrorEnum] on failure.
  factory ResultOpaque(int i) {
    final result = _ResultOpaque_new(i);
    if (!result.isOk) {
      throw ErrorEnum.values[result.union.err];
    }
    return ResultOpaque._fromFfi(result.union.ok, []);
  }

  ///
  ///
  /// Throws [ErrorEnum] on failure.
  factory ResultOpaque.failingFoo() {
    final result = _ResultOpaque_new_failing_foo();
    if (!result.isOk) {
      throw ErrorEnum.values[result.union.err];
    }
    return ResultOpaque._fromFfi(result.union.ok, []);
  }

  ///
  ///
  /// Throws [ErrorEnum] on failure.
  factory ResultOpaque.failingBar() {
    final result = _ResultOpaque_new_failing_bar();
    if (!result.isOk) {
      throw ErrorEnum.values[result.union.err];
    }
    return ResultOpaque._fromFfi(result.union.ok, []);
  }

  static ResultOpaque? failingUnit() {
    final result = _ResultOpaque_new_failing_unit();
    if (!result.isOk) {
      return null;
    }
    return ResultOpaque._fromFfi(result.union.ok, []);
  }

  ///
  ///
  /// Throws [ErrorStruct] on failure.
  factory ResultOpaque.failingStruct(int i) {
    final result = _ResultOpaque_new_failing_struct(i);
    if (!result.isOk) {
      throw ErrorStruct._fromFfi(result.union.err);
    }
    return ResultOpaque._fromFfi(result.union.ok, []);
  }

  ///
  ///
  /// Throws [ResultOpaque] on failure.
  static void newInErr(int i) {
    final result = _ResultOpaque_new_in_err(i);
    if (!result.isOk) {
      throw ResultOpaque._fromFfi(result.union.err, []);
    }
  }

  static int? newInt(int i) {
    final result = _ResultOpaque_new_int(i);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok;
  }

  ///
  ///
  /// Throws [ResultOpaque] on failure.
  static ErrorEnum newInEnumErr(int i) {
    final result = _ResultOpaque_new_in_enum_err(i);
    if (!result.isOk) {
      throw ResultOpaque._fromFfi(result.union.err, []);
    }
    return ErrorEnum.values[result.union.ok];
  }

  /// When we take &str, the return type becomes a Result
  /// Test that this interacts gracefully with returning a reference type
  ResultOpaque takesStr(String v) {
    final temp = _FinalizedArena();
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> aEdges = [this];
    final result = _ResultOpaque_takes_str(_ffi, v._utf8AllocIn(temp.arena));
    return ResultOpaque._fromFfi(result, aEdges);
  }

  void assertInteger(int i) {
    _ResultOpaque_assert_integer(_ffi, i);
  }

}

@_DiplomatFfiUse('ResultOpaque_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ResultOpaque_destroy')
// ignore: non_constant_identifier_names
external void _ResultOpaque_destroy(ffi.Pointer<ffi.Void> self);

@_DiplomatFfiUse('ResultOpaque_new')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_new')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ResultOpaque_new(int i);

@_DiplomatFfiUse('ResultOpaque_new_failing_foo')
@ffi.Native<_ResultOpaqueInt32 Function()>(isLeaf: true, symbol: 'ResultOpaque_new_failing_foo')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ResultOpaque_new_failing_foo();

@_DiplomatFfiUse('ResultOpaque_new_failing_bar')
@ffi.Native<_ResultOpaqueInt32 Function()>(isLeaf: true, symbol: 'ResultOpaque_new_failing_bar')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ResultOpaque_new_failing_bar();

@_DiplomatFfiUse('ResultOpaque_new_failing_unit')
@ffi.Native<_ResultOpaqueVoid Function()>(isLeaf: true, symbol: 'ResultOpaque_new_failing_unit')
// ignore: non_constant_identifier_names
external _ResultOpaqueVoid _ResultOpaque_new_failing_unit();

@_DiplomatFfiUse('ResultOpaque_new_failing_struct')
@ffi.Native<_ResultOpaqueErrorStructFfi Function(ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_new_failing_struct')
// ignore: non_constant_identifier_names
external _ResultOpaqueErrorStructFfi _ResultOpaque_new_failing_struct(int i);

@_DiplomatFfiUse('ResultOpaque_new_in_err')
@ffi.Native<_ResultVoidOpaque Function(ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_new_in_err')
// ignore: non_constant_identifier_names
external _ResultVoidOpaque _ResultOpaque_new_in_err(int i);

@_DiplomatFfiUse('ResultOpaque_new_int')
@ffi.Native<_ResultInt32Void Function(ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_new_int')
// ignore: non_constant_identifier_names
external _ResultInt32Void _ResultOpaque_new_int(int i);

@_DiplomatFfiUse('ResultOpaque_new_in_enum_err')
@ffi.Native<_ResultInt32Opaque Function(ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_new_in_enum_err')
// ignore: non_constant_identifier_names
external _ResultInt32Opaque _ResultOpaque_new_in_enum_err(int i);

@_DiplomatFfiUse('ResultOpaque_takes_str')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, _SliceUtf8)>(isLeaf: true, symbol: 'ResultOpaque_takes_str')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ResultOpaque_takes_str(ffi.Pointer<ffi.Opaque> self, _SliceUtf8 v);

@_DiplomatFfiUse('ResultOpaque_assert_integer')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_assert_integer')
// ignore: non_constant_identifier_names
external void _ResultOpaque_assert_integer(ffi.Pointer<ffi.Opaque> self, int i);

// dart format on
