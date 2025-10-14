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

    var maybeU8 = OptionOpaque.acceptsOptionU8(123);
    expect(maybeU8, null);
    maybeU8 = OptionOpaque.acceptsOptionU8(123, 5);
    expect(maybeU8, 5);

    var maybeEnum = OptionOpaque.acceptsOptionEnum(123);
    expect(maybeEnum, null);
    maybeEnum = OptionOpaque.acceptsOptionEnum(123, OptionEnum.foo);
    expect(maybeEnum, OptionEnum.foo);

    var maybeStruct = OptionOpaque.acceptsOptionInputStruct(123);
    expect(maybeStruct, null);
    maybeStruct = OptionOpaque.acceptsOptionInputStruct(
      123,
      new OptionInputStruct(a: 7, b: null, c: OptionEnum.bar),
    );
    expect(maybeStruct?.a, 7);
    expect(maybeStruct?.b, null);
    expect(maybeStruct?.c, OptionEnum.bar);

    final struct = OptionOpaque.returnsOptionInputStruct();
    expect(struct.a, 6);
    expect(struct.b, null);
    expect(struct.c, OptionEnum.bar);

    final borrowed = new BorrowingOptionStruct(a: "test string");
    OptionOpaque.acceptsBorrowingOptionStruct(borrowed);
  });
}
