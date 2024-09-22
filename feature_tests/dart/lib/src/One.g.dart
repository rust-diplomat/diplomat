// generated by diplomat-tool

part of 'lib.g.dart';

final class One implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;
  // ignore: unused_field
  final core.List<Object> _aEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  One._fromFfi(this._ffi, this._selfEdge, this._aEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_One_destroy));

  factory One.transitivity(One hold, One nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c, 'd, 'e
    core.List<Object> aEdges = [hold];
    final result = _One_transitivity(hold._ffi, nohold._ffi);
    return One._fromFfi(result, [], aEdges);
  }

  factory One.cycle(Two hold, One nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c
    core.List<Object> aEdges = [hold];
    final result = _One_cycle(hold._ffi, nohold._ffi);
    return One._fromFfi(result, [], aEdges);
  }

  factory One.manyDependents(One a, One b, Two c, Two d, Two nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c, 'd
    core.List<Object> aEdges = [a, b, c, d];
    final result = _One_many_dependents(a._ffi, b._ffi, c._ffi, d._ffi, nohold._ffi);
    return One._fromFfi(result, [], aEdges);
  }

  factory One.returnOutlivesParam(Two hold, One nohold) {
    // This lifetime edge depends on lifetimes: 'long
    core.List<Object> longEdges = [hold];
    final result = _One_return_outlives_param(hold._ffi, nohold._ffi);
    return One._fromFfi(result, [], longEdges);
  }

  factory One.diamondTop(One top, One left, One right, One bottom) {
    // This lifetime edge depends on lifetimes: 'top, 'left, 'right, 'bottom
    core.List<Object> topEdges = [top, left, right, bottom];
    final result = _One_diamond_top(top._ffi, left._ffi, right._ffi, bottom._ffi);
    return One._fromFfi(result, [], topEdges);
  }

  factory One.diamondLeft(One top, One left, One right, One bottom) {
    // This lifetime edge depends on lifetimes: 'left, 'bottom
    core.List<Object> leftEdges = [left, bottom];
    final result = _One_diamond_left(top._ffi, left._ffi, right._ffi, bottom._ffi);
    return One._fromFfi(result, [], leftEdges);
  }

  factory One.diamondRight(One top, One left, One right, One bottom) {
    // This lifetime edge depends on lifetimes: 'right, 'bottom
    core.List<Object> rightEdges = [right, bottom];
    final result = _One_diamond_right(top._ffi, left._ffi, right._ffi, bottom._ffi);
    return One._fromFfi(result, [], rightEdges);
  }

  factory One.diamondBottom(One top, One left, One right, One bottom) {
    // This lifetime edge depends on lifetimes: 'bottom
    core.List<Object> bottomEdges = [bottom];
    final result = _One_diamond_bottom(top._ffi, left._ffi, right._ffi, bottom._ffi);
    return One._fromFfi(result, [], bottomEdges);
  }

  factory One.diamondAndNestedTypes(One a, One b, One c, One d, One nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c, 'd
    core.List<Object> aEdges = [a, b, c, d];
    final result = _One_diamond_and_nested_types(a._ffi, b._ffi, c._ffi, d._ffi, nohold._ffi);
    return One._fromFfi(result, [], aEdges);
  }

  factory One.implicitBounds(One explicitHold, One implicitHold, One nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c, 'd, 'x
    core.List<Object> aEdges = [explicitHold, implicitHold];
    final result = _One_implicit_bounds(explicitHold._ffi, implicitHold._ffi, nohold._ffi);
    return One._fromFfi(result, [], aEdges);
  }

  factory One.implicitBoundsDeep(One explicit, One implicit1, One implicit2, One nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c, 'd
    core.List<Object> aEdges = [explicit, implicit1, implicit2];
    final result = _One_implicit_bounds_deep(explicit._ffi, implicit1._ffi, implicit2._ffi, nohold._ffi);
    return One._fromFfi(result, [], aEdges);
  }
}

@meta.RecordUse('One_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'One_destroy')
// ignore: non_constant_identifier_names
external void _One_destroy(ffi.Pointer<ffi.Void> self);

@meta.RecordUse('One_transitivity')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_transitivity')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_transitivity(ffi.Pointer<ffi.Opaque> hold, ffi.Pointer<ffi.Opaque> nohold);

@meta.RecordUse('One_cycle')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_cycle')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_cycle(ffi.Pointer<ffi.Opaque> hold, ffi.Pointer<ffi.Opaque> nohold);

@meta.RecordUse('One_many_dependents')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_many_dependents')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_many_dependents(ffi.Pointer<ffi.Opaque> a, ffi.Pointer<ffi.Opaque> b, ffi.Pointer<ffi.Opaque> c, ffi.Pointer<ffi.Opaque> d, ffi.Pointer<ffi.Opaque> nohold);

@meta.RecordUse('One_return_outlives_param')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_return_outlives_param')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_return_outlives_param(ffi.Pointer<ffi.Opaque> hold, ffi.Pointer<ffi.Opaque> nohold);

@meta.RecordUse('One_diamond_top')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_top')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_top(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@meta.RecordUse('One_diamond_left')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_left')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_left(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@meta.RecordUse('One_diamond_right')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_right')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_right(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@meta.RecordUse('One_diamond_bottom')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_bottom')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_bottom(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@meta.RecordUse('One_diamond_and_nested_types')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_and_nested_types')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_and_nested_types(ffi.Pointer<ffi.Opaque> a, ffi.Pointer<ffi.Opaque> b, ffi.Pointer<ffi.Opaque> c, ffi.Pointer<ffi.Opaque> d, ffi.Pointer<ffi.Opaque> nohold);

@meta.RecordUse('One_implicit_bounds')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_implicit_bounds')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_implicit_bounds(ffi.Pointer<ffi.Opaque> explicitHold, ffi.Pointer<ffi.Opaque> implicitHold, ffi.Pointer<ffi.Opaque> nohold);

@meta.RecordUse('One_implicit_bounds_deep')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_implicit_bounds_deep')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_implicit_bounds_deep(ffi.Pointer<ffi.Opaque> explicit, ffi.Pointer<ffi.Opaque> implicit1, ffi.Pointer<ffi.Opaque> implicit2, ffi.Pointer<ffi.Opaque> nohold);
