import 'package:feature_tests/lib.dart';
import 'package:test/test.dart';

void main() {
  test('Verify invariants of struct', () {
    final s = MyStruct();
    expect(s.a, 17);
    expect(s.b, true);
    expect(s.c, 209);
    expect(s.d, 1234);
    expect(s.e, 5991);
    expect(s.f, '餐'.runes.first);
    expect(s.g, MyEnum.b);
  });

  test('Verify ImmutableStructOfOpaque', () {
    final op = Opaque.fromStr('String');
    final immut = ImmutableStructOfOpaque(i: op);
    expect(immut.takeIn(), '"String"');
  });

  test('Utf16Wrap writeTo and writeChars', () {
    final w = Utf16Wrap('Hello 𐐷!');
    expect(w.writeTo(), 'Hello 𐐷!');
    expect(w.writeChars(), 'Hello 𐐷!');
    final empty = Utf16Wrap('');
    expect(empty.writeTo(), '');
  });
}
