import 'Two.dart';
import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef OneFfi = ffi.Pointer<ffi.Opaque>;

class One implements ffi.Finalizable {
  final OneFfi _underlying;

  factory One.transitivity(One hold, One nohold) {
    final result = _transitivityFfi(OneAsFfi(hold), OneAsFfi(nohold));
    return OneFromFfi(result);
  }
  static late final _transitivityFfi =
      capi<ffi.NativeFunction<OneFfi Function(OneFfi, OneFfi)>>(
              'One_transitivity')
          .asFunction<OneFfi Function(OneFfi, OneFfi)>();

  factory One.cycle(Two hold, One nohold) {
    final result = _cycleFfi(TwoAsFfi(hold), OneAsFfi(nohold));
    return OneFromFfi(result);
  }
  static late final _cycleFfi =
      capi<ffi.NativeFunction<OneFfi Function(TwoFfi, OneFfi)>>('One_cycle')
          .asFunction<OneFfi Function(TwoFfi, OneFfi)>();

  factory One.manyDependents(One a, One b, Two c, Two d, Two nohold) {
    final result = _manyDependentsFfi(
        OneAsFfi(a), OneAsFfi(b), TwoAsFfi(c), TwoAsFfi(d), TwoAsFfi(nohold));
    return OneFromFfi(result);
  }
  static late final _manyDependentsFfi = capi<
          ffi.NativeFunction<
              OneFfi Function(OneFfi, OneFfi, TwoFfi, TwoFfi,
                  TwoFfi)>>('One_many_dependents')
      .asFunction<OneFfi Function(OneFfi, OneFfi, TwoFfi, TwoFfi, TwoFfi)>();

  factory One.returnOutlivesParam(Two hold, One nohold) {
    final result = _returnOutlivesParamFfi(TwoAsFfi(hold), OneAsFfi(nohold));
    return OneFromFfi(result);
  }
  static late final _returnOutlivesParamFfi =
      capi<ffi.NativeFunction<OneFfi Function(TwoFfi, OneFfi)>>(
              'One_return_outlives_param')
          .asFunction<OneFfi Function(TwoFfi, OneFfi)>();

  factory One.diamondTop(One top, One left, One right, One bottom) {
    final result = _diamondTopFfi(
        OneAsFfi(top), OneAsFfi(left), OneAsFfi(right), OneAsFfi(bottom));
    return OneFromFfi(result);
  }
  static late final _diamondTopFfi =
      capi<ffi.NativeFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi)>>(
              'One_diamond_top')
          .asFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi)>();

  factory One.diamondLeft(One top, One left, One right, One bottom) {
    final result = _diamondLeftFfi(
        OneAsFfi(top), OneAsFfi(left), OneAsFfi(right), OneAsFfi(bottom));
    return OneFromFfi(result);
  }
  static late final _diamondLeftFfi =
      capi<ffi.NativeFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi)>>(
              'One_diamond_left')
          .asFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi)>();

  factory One.diamondRight(One top, One left, One right, One bottom) {
    final result = _diamondRightFfi(
        OneAsFfi(top), OneAsFfi(left), OneAsFfi(right), OneAsFfi(bottom));
    return OneFromFfi(result);
  }
  static late final _diamondRightFfi =
      capi<ffi.NativeFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi)>>(
              'One_diamond_right')
          .asFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi)>();

  factory One.diamondBottom(One top, One left, One right, One bottom) {
    final result = _diamondBottomFfi(
        OneAsFfi(top), OneAsFfi(left), OneAsFfi(right), OneAsFfi(bottom));
    return OneFromFfi(result);
  }
  static late final _diamondBottomFfi =
      capi<ffi.NativeFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi)>>(
              'One_diamond_bottom')
          .asFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi)>();

  factory One.diamondAndNestedTypes(One a, One b, One c, One d, One nohold) {
    final result = _diamondAndNestedTypesFfi(
        OneAsFfi(a), OneAsFfi(b), OneAsFfi(c), OneAsFfi(d), OneAsFfi(nohold));
    return OneFromFfi(result);
  }
  static late final _diamondAndNestedTypesFfi = capi<
          ffi.NativeFunction<
              OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi,
                  OneFfi)>>('One_diamond_and_nested_types')
      .asFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi, OneFfi)>();

  factory One.implicitBounds(One explicitHold, One implicitHold, One nohold) {
    final result = _implicitBoundsFfi(
        OneAsFfi(explicitHold), OneAsFfi(implicitHold), OneAsFfi(nohold));
    return OneFromFfi(result);
  }
  static late final _implicitBoundsFfi =
      capi<ffi.NativeFunction<OneFfi Function(OneFfi, OneFfi, OneFfi)>>(
              'One_implicit_bounds')
          .asFunction<OneFfi Function(OneFfi, OneFfi, OneFfi)>();

  factory One.implicitBoundsDeep(
      One explicit, One implicit1, One implicit2, One nohold) {
    final result = _implicitBoundsDeepFfi(OneAsFfi(explicit),
        OneAsFfi(implicit1), OneAsFfi(implicit2), OneAsFfi(nohold));
    return OneFromFfi(result);
  }
  static late final _implicitBoundsDeepFfi =
      capi<ffi.NativeFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi)>>(
              'One_implicit_bounds_deep')
          .asFunction<OneFfi Function(OneFfi, OneFfi, OneFfi, OneFfi)>();

  One._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'One_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
One OneFromFfi(OneFfi underlying) => One._(underlying);
OneFfi OneAsFfi(One t) => t._underlying;
