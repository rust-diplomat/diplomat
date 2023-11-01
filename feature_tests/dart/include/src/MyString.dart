import 'dart:convert';
import 'dart:ffi' as ffi;
import 'diplomat_runtime.dart';
import 'package:ffi/ffi.dart' as allocators;

typedef MyStringFfi = ffi.Pointer<ffi.Opaque>;

class MyString implements ffi.Finalizable {
  final MyStringFfi _underlying;

  factory MyString.new(String v) {
    final alloc = allocators.Arena();

    final vList = Utf8Encoder().convert(v);
    final vBytes = alloc.call<ffi.Char>(vList.length);
    vBytes.cast<ffi.Uint8>().asTypedList(vList.length).setAll(0, vList);

    final result = _newFfi(vBytes.cast(), vList.length);
    alloc.releaseAll();
    return MyStringFromFfi(result);
  }
  static late final _newFfi = capi<
          ffi.NativeFunction<
              MyStringFfi Function(
                  ffi.Pointer<ffi.Char>, ffi.Size)>>('MyString_new')
      .asFunction<MyStringFfi Function(ffi.Pointer<ffi.Char>, int)>();

  void setStr(String newStr) {
    final alloc = allocators.Arena();

    final newStrList = Utf8Encoder().convert(newStr);
    final newStrBytes = alloc.call<ffi.Char>(newStrList.length);
    newStrBytes
        .cast<ffi.Uint8>()
        .asTypedList(newStrList.length)
        .setAll(0, newStrList);

    _setStrFfi(this._underlying, newStrBytes.cast(), newStrList.length);
    alloc.releaseAll();
  }

  static late final _setStrFfi = capi<
          ffi.NativeFunction<
              ffi.Void Function(MyStringFfi, ffi.Pointer<ffi.Char>,
                  ffi.Size)>>('MyString_set_str')
      .asFunction<void Function(MyStringFfi, ffi.Pointer<ffi.Char>, int)>();

  String getStr() {
    final writeable = create_writeable();
    _getStrFfi(this._underlying, writeable);
    return writeable_to_string(writeable);
  }

  static late final _getStrFfi = capi<
          ffi.NativeFunction<
              ffi.Void Function(
                  MyStringFfi, DiplomatWriteable)>>('MyString_get_str')
      .asFunction<void Function(MyStringFfi, DiplomatWriteable)>();

  MyString._(this._underlying) {
    _finalizer.attach(this, this._underlying.cast());
  }

  static late final _finalizer = ffi.NativeFinalizer(
      capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'MyString_destroy'));
}

// These are not methods because we want to keep them package-private, and methods are either private or public
MyString MyStringFromFfi(MyStringFfi underlying) => MyString._(underlying);
MyStringFfi MyStringAsFfi(MyString t) => t._underlying;
