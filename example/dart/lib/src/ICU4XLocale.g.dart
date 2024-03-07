// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X Locale, capable of representing strings like `"en-US"`.
///
/// See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
final class ICU4XLocale implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // isOwned is whether this is owned (has finalizer) or not
  // This also takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  ICU4XLocale._fromFfi(this._ffi, bool isOwned, this._selfEdge) {
    if (isOwned) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XLocale_destroy));

  /// Construct an [`ICU4XLocale`] from a locale identifier represented as a string.
  factory ICU4XLocale(String name) {
    final temp = ffi2.Arena();
    final nameView = name.utf8View;
    final result = _ICU4XLocale_new(nameView.allocIn(temp), nameView.length);
    temp.releaseAll();
    return ICU4XLocale._fromFfi(result, true, []);
  }
}

@meta.ResourceIdentifier('ICU4XLocale_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XLocale_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XLocale_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier('ICU4XLocale_new')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XLocale_new')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XLocale_new(ffi.Pointer<ffi.Uint8> nameData, int nameLength);
