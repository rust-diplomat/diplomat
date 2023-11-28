import 'package:feature_tests/lib.dart';
import 'package:test/test.dart';
import 'package:path/path.dart' as path;

void main() {
  init(path.absolute('../../target/debug/libdiplomat_feature_tests.so'));

  test("Verify option methods", () {
    final o = OptionOpaque.new_(5);
    o!.assertInteger(5);

    final on = OptionOpaque.none;
    expect(on, null);

    final s = OptionOpaque.struct;

    s.a!.assertInteger(101);
    s.b!.assertChar('餐'.runes.first);
    expect(s.c, 904);
    s.d!.assertInteger(926535);

    final sn = OptionOpaque.structNones;
    expect(sn.a, null);
    expect(sn.b, null);
    expect(sn.c, 908);
    expect(sn.d, null);
  });
}
