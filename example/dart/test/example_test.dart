import 'package:example/lib.dart';
import 'package:test/test.dart';

void main() {
  test("multiply a fixed decimal by 0.1", () {
    final myDecimal = FixedDecimal.new(123);

    myDecimal.multiplyPow10(-1);
    expect(myDecimal.toStringFallible(), "12.3");
  });

  test("format a fixed decimal", () {
    final myDecimal = FixedDecimal.new(123);

    myDecimal.multiplyPow10(-1);

    final locale = Locale.new("bn");

    final dataProvider = DataProvider.static_();

    final fdf = FixedDecimalFormatter.tryNew(
      locale,
      dataProvider,
      FixedDecimalFormatterOptions(),
    )!;

    expect(fdf.formatWrite(myDecimal), "১২.৩");
  });
}
