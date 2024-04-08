import 'package:feature_tests/lib.dart';
import 'package:test/test.dart';

void main() {
  test("Verify option methods", () {
    OptionOpaque.new_(5)!.assertInteger(5);

    expect(OptionOpaque.newNone(), null);

    final s = OptionOpaque.newStruct();
    s.a!.assertInteger(101);
    s.b!.assertChar('餐'.runes.first);
    expect(s.c, 904);
    s.d!.assertInteger(926535);

    final sn = OptionOpaque.newStructNones();
    expect(sn.a, null);
    expect(sn.b, null);
    expect(sn.c, 908);
    expect(sn.d, null);
  });
}
