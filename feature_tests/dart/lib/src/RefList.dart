import 'RefListParameter.dart';
import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef RefListFfi = ffi.Pointer<ffi.Opaque>;

class RefList implements ffi.Finalizable {
  final RefListFfi _underlying;

  factory RefList.node(RefListParameter data) {
    final result = _nodeFfi(RefListParameterAsFfi(data));
    return RefListFromFfi(result);
  }
  static late final _nodeFfi =
      capi<ffi.NativeFunction<RefListFfi Function(RefListParameterFfi)>>(
              'RefList_node')
          .asFunction<RefListFfi Function(RefListParameterFfi)>();

  RefList._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'RefList_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
RefList RefListFromFfi(RefListFfi underlying) => RefList._(underlying);
RefListFfi RefListAsFfi(RefList t) => t._underlying;
