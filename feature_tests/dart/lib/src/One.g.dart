// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class One implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  final core.List<Object> _edge_self;
  final core.List<Object> _edge_a;

  // Internal constructor from FFI.
  // isOwned is whether this is owned (has finalizer) or not
  // This also takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  One._(this._underlying, bool isOwned, this._edge_self, this._edge_a) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_One_destroy));

  factory One.transitivity(One hold, One nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c, 'd, 'e
    core.List<Object> aEdges = [hold];
    final result = _One_transitivity(hold._underlying, nohold._underlying);
    return One._(result, true, [], aEdges);
  }

  factory One.cycle(Two hold, One nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c
    core.List<Object> aEdges = [hold];
    final result = _One_cycle(hold._underlying, nohold._underlying);
    return One._(result, true, [], aEdges);
  }

  factory One.manyDependents(One a, One b, Two c, Two d, Two nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c, 'd
    core.List<Object> aEdges = [a, b, c, d];
    final result = _One_many_dependents(a._underlying, b._underlying, c._underlying, d._underlying, nohold._underlying);
    return One._(result, true, [], aEdges);
  }

  factory One.returnOutlivesParam(Two hold, One nohold) {
    // This lifetime edge depends on lifetimes: 'long
    core.List<Object> longEdges = [hold];
    final result = _One_return_outlives_param(hold._underlying, nohold._underlying);
    return One._(result, true, [], longEdges);
  }

  factory One.diamondTop(One top, One left, One right, One bottom) {
    // This lifetime edge depends on lifetimes: 'top, 'left, 'right, 'bottom
    core.List<Object> topEdges = [top, left, right, bottom];
    final result = _One_diamond_top(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result, true, [], topEdges);
  }

  factory One.diamondLeft(One top, One left, One right, One bottom) {
    // This lifetime edge depends on lifetimes: 'left, 'bottom
    core.List<Object> leftEdges = [left, bottom];
    final result = _One_diamond_left(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result, true, [], leftEdges);
  }

  factory One.diamondRight(One top, One left, One right, One bottom) {
    // This lifetime edge depends on lifetimes: 'right, 'bottom
    core.List<Object> rightEdges = [right, bottom];
    final result = _One_diamond_right(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result, true, [], rightEdges);
  }

  factory One.diamondBottom(One top, One left, One right, One bottom) {
    // This lifetime edge depends on lifetimes: 'bottom
    core.List<Object> bottomEdges = [bottom];
    final result = _One_diamond_bottom(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result, true, [], bottomEdges);
  }

  factory One.diamondAndNestedTypes(One a, One b, One c, One d, One nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c, 'd
    core.List<Object> aEdges = [a, b, c, d];
    final result = _One_diamond_and_nested_types(a._underlying, b._underlying, c._underlying, d._underlying, nohold._underlying);
    return One._(result, true, [], aEdges);
  }

  factory One.implicitBounds(One explicitHold, One implicitHold, One nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c, 'd, 'x
    core.List<Object> aEdges = [explicitHold, implicitHold];
    final result = _One_implicit_bounds(explicitHold._underlying, implicitHold._underlying, nohold._underlying);
    return One._(result, true, [], aEdges);
  }

  factory One.implicitBoundsDeep(One explicit, One implicit1, One implicit2, One nohold) {
    // This lifetime edge depends on lifetimes: 'a, 'b, 'c, 'd
    core.List<Object> aEdges = [explicit, implicit1, implicit2];
    final result = _One_implicit_bounds_deep(explicit._underlying, implicit1._underlying, implicit2._underlying, nohold._underlying);
    return One._(result, true, [], aEdges);
  }
}

@meta.ResourceIdentifier()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'One_destroy')
// ignore: non_constant_identifier_names
external void _One_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_transitivity')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_transitivity(ffi.Pointer<ffi.Opaque> hold, ffi.Pointer<ffi.Opaque> nohold);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_cycle')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_cycle(ffi.Pointer<ffi.Opaque> hold, ffi.Pointer<ffi.Opaque> nohold);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_many_dependents')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_many_dependents(ffi.Pointer<ffi.Opaque> a, ffi.Pointer<ffi.Opaque> b, ffi.Pointer<ffi.Opaque> c, ffi.Pointer<ffi.Opaque> d, ffi.Pointer<ffi.Opaque> nohold);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_return_outlives_param')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_return_outlives_param(ffi.Pointer<ffi.Opaque> hold, ffi.Pointer<ffi.Opaque> nohold);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_top')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_top(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_left')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_left(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_right')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_right(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_bottom')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_bottom(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_and_nested_types')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_and_nested_types(ffi.Pointer<ffi.Opaque> a, ffi.Pointer<ffi.Opaque> b, ffi.Pointer<ffi.Opaque> c, ffi.Pointer<ffi.Opaque> d, ffi.Pointer<ffi.Opaque> nohold);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_implicit_bounds')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_implicit_bounds(ffi.Pointer<ffi.Opaque> explicitHold, ffi.Pointer<ffi.Opaque> implicitHold, ffi.Pointer<ffi.Opaque> nohold);

@meta.ResourceIdentifier()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_implicit_bounds_deep')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_implicit_bounds_deep(ffi.Pointer<ffi.Opaque> explicit, ffi.Pointer<ffi.Opaque> implicit1, ffi.Pointer<ffi.Opaque> implicit2, ffi.Pointer<ffi.Opaque> nohold);
