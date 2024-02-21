import 'package:example/lib.dart';
import 'package:test/test.dart';

void main() {
  test("multiply a fixed decimal by 0.1", () {
    final myDecimal = ICU4XFixedDecimal.new(123);

    myDecimal.multiplyPow10(-1);
    expect(myDecimal.toStringFallible(), "12.3");
  });

  test("format a fixed decimal", () {
    final myDecimal = ICU4XFixedDecimal.new(123);

    myDecimal.multiplyPow10(-1);

    final locale = ICU4XLocale.new("bn");

    final dataProvider = ICU4XDataProvider.static_();

    final fdf = ICU4XFixedDecimalFormatter.tryNew(
        locale, dataProvider, ICU4XFixedDecimalFormatterOptions())!;

    expect(fdf.formatWrite(myDecimal), "১২.৩");
  });
}
