import 'package:feature_tests/lib.dart';
import 'package:test/test.dart';

void main() {
  test('Verify iterator behavior', () {
    final it = RenamedMyIterable([10, 20, 30, 40, 50]);

    var next = 10;
    for (final i in it) {
      expect(i, next);
      next += 10;
    }

    expect(next, 60);
  });
}
