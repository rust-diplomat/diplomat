import 'package:feature_tests/lib.dart';
import 'package:test/test.dart';
import 'dart:typed_data';

void main() {
  test("double", () {
    expect(
      Float64Vec([-10.0, double.maxFinite, double.infinity]).toString(),
      "[-10.0, 1.7976931348623157e308, inf]",
    );
  });

  test("isize", () {
    // max integer value
    expect(
      Float64Vec.isize([
        -9223372036854775808,
        -1,
        9223372036854775807,
      ]).toString(),
      "[-9.223372036854776e18, -1.0, 9.223372036854776e18]",
    );
  });

  test("usize", () {
    expect(
      Float64Vec.usize([
        -9223372036854775808,
        -1,
        9223372036854775807,
      ]).toString(),
      "[0.0, 0.0, 9.223372036854776e18]",
    );
  });

  test("i16", () {
    expect(Float64Vec.i16([-10, 10]).toString(), "[-10.0, 10.0]");
  });

  test("u16", () {
    expect(Float64Vec.u16([-10, 10]).toString(), "[0.0, 10.0]");
  });

  test("bool", () {
    expect(Float64Vec.bool([true, false]).toString(), "[1.0, 0.0]");
  });

  test("bytes", () {
    expect(
      Float64Vec.f64BeBytes(
        Uint8List.fromList([64, 40, 174, 20, 122, 225, 71, 174]).buffer,
      ).toString(),
      "[12.34]",
    );
  });

  test("strings", () {
    final s = MyString.newFromFirst(["foo", "bar"]);
    expect(s.str, "foo");
  });
}
