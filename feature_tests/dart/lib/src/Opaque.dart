import 'ImportedStruct.dart';
import 'MyStruct.dart';
import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';

typedef OpaqueFfi = ffi.Pointer<ffi.Opaque>;

class Opaque implements ffi.Finalizable {
  final OpaqueFfi _underlying;

  factory Opaque.new() {
    final result = _newFfi();
    return OpaqueFromFfi(result);
  }
  static late final _newFfi =
      capi<ffi.NativeFunction<OpaqueFfi Function()>>('Opaque_new')
          .asFunction<OpaqueFfi Function()>();

  /// See the [Rust documentation for `something`](https://docs.rs/Something/latest/struct.Something.html#method.something) for more information.
  ///
  /// See the [Rust documentation for `something_else`](https://docs.rs/Something/latest/struct.Something.html#method.something_else) for more information.
  ///
  /// Additional information: [1](https://docs.rs/Something/latest/struct.Something.html#method.something_small), [2](https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something)
  void assertStruct(MyStruct s) {
    _assertStructFfi(this._underlying, MyStructAsFfi(s));
  }

  static late final _assertStructFfi =
      capi<ffi.NativeFunction<ffi.Void Function(OpaqueFfi, MyStructFfi)>>(
              'Opaque_assert_struct')
          .asFunction<void Function(OpaqueFfi, MyStructFfi)>();

  static int returnsUsize() {
    final result = _returnsUsizeFfi();
    return result;
  }

  static late final _returnsUsizeFfi =
      capi<ffi.NativeFunction<ffi.Uint64 Function()>>('Opaque_returns_usize')
          .asFunction<int Function()>();

  static ImportedStruct returnsImported() {
    final result = _returnsImportedFfi();
    return ImportedStructFromFfi(result);
  }

  static late final _returnsImportedFfi =
      capi<ffi.NativeFunction<ImportedStructFfi Function()>>(
              'Opaque_returns_imported')
          .asFunction<ImportedStructFfi Function()>();

  Opaque._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'Opaque_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
Opaque OpaqueFromFfi(OpaqueFfi underlying) => Opaque._(underlying);
OpaqueFfi OpaqueAsFfi(Opaque t) => t._underlying;
