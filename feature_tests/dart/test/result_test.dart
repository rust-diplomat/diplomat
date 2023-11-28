import 'package:feature_tests/lib.dart';
import 'package:test/test.dart';
import 'package:path/path.dart' as path;

void main() {
    init(path.absolute('../../target/debug/libdiplomat_feature_tests.so'));

    test("Verify result methods", () {
        expect(ResultOpaque(5), ResultOpaqueMatcher(5));

        expect(ResultOpaque.failingFoo, throwsA(ErrorEnum.foo));
        expect(ResultOpaque.failingBar, throwsA(ErrorEnum.bar));
        expect(ResultOpaque.failingUnit, throwsA(VoidError()));
        expect(() => ResultOpaque.failingStruct(109), throwsA(ErrorStruct()..i = 109..j = 12));

        expect(() => ResultOpaque.newInErr(559), throwsA(ResultOpaqueMatcher(559)));
        expect(() => ResultOpaque.newInEnumErr(881), throwsA(ResultOpaqueMatcher(881)));

    });
}

class ResultOpaqueMatcher extends Matcher {
    final int val;
    ResultOpaqueMatcher(this.val);
    @override
    bool matches(dynamic item, Map matchState) {
      if (item is ResultOpaque) {
        item.assertInteger(val);
        return true;
      } else {
        return false;
      }
    }
    @override
    Description describe(Description description) => description;
}