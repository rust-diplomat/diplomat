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
    s.d.assertInteger(926535);

    final sn = OptionOpaque.newStructNones();
    expect(sn.a, null);
    expect(sn.b, null);
    expect(sn.c, 908);

    var maybeU8 = OptionOpaque.acceptsOptionU8(null);
    expect(maybeU8, null);
    maybeU8 = OptionOpaque.acceptsOptionU8(5);
    expect(maybeU8, 5);

    var maybeEnum = OptionOpaque.acceptsOptionEnum(null);
    expect(maybeEnum, null);
    maybeEnum = OptionOpaque.acceptsOptionEnum(OptionEnum.foo);
    expect(maybeEnum, OptionEnum.foo);

    var maybeStruct = OptionOpaque.acceptsOptionInputStruct(null);
    expect(maybeStruct, null);
    maybeStruct = OptionOpaque.acceptsOptionInputStruct(
      new OptionInputStruct(a: 7, b: null, c: OptionEnum.bar),
    );
    expect(maybeStruct?.a, 7);
    expect(maybeStruct?.b, null);
    expect(maybeStruct?.c, OptionEnum.bar);

    final struct = OptionOpaque.returnsOptionInputStruct();
    expect(struct.a, 6);
    expect(struct.b, null);
    expect(struct.c, OptionEnum.bar);
  });
}
