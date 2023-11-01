import 'OptionOpaque.dart';
import 'OptionOpaqueChar.dart';
import 'dart:ffi' as ffi;


class OptionStructFfi extends ffi.Struct {
    external OptionOpaqueFfi a;
    external OptionOpaqueCharFfi b;
    @ffi.Uint32()
    external int c;
    external OptionOpaqueFfi d;
}

class OptionStruct {
  final OptionStructFfi _underlying;
  OptionOpaque? get a => this._underlying.a.address == 0 ? null : OptionOpaqueFromFfi(this._underlying.a);
  void set a(OptionOpaque? a) {this._underlying.a = OptionOpaque?AsFfi(a);
  }

  OptionOpaqueChar? get b => this._underlying.b.address == 0 ? null : OptionOpaqueCharFromFfi(this._underlying.b);
  void set b(OptionOpaqueChar? b) {this._underlying.b = OptionOpaqueChar?AsFfi(b);
  }

  int get c => this._underlying.c;
  void set c(int c) {this._underlying.c = c;
  }

  OptionOpaque? get d => this._underlying.d.address == 0 ? null : OptionOpaqueFromFfi(this._underlying.d);
  void set d(OptionOpaque? d) {this._underlying.d = OptionOpaque?AsFfi(d);
  }

  OptionStruct._(this._underlying);
}

// These are not methods because we want to keep them package-private, and methods are either private or public
OptionStruct OptionStructFromFfi(OptionStructFfi underlying) => OptionStruct._(underlying);
OptionStructFfi OptionStructAsFfi(OptionStruct t) => t._underlying;

