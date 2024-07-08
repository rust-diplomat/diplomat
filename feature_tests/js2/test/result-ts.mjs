import test from 'ava';
import { ResultOpaque } from "diplomat-wasm-js2-feature-tests";
test("Verify result methods", t => {
    const s = ResultOpaque.new_(5);
    s.assertInteger(5);
    const error_foo = t.throws(() => ResultOpaque.newFailingFoo());
    t.is(error_foo.error_value.value, "Foo");
    const error_bar = t.throws(() => ResultOpaque.newFailingBar());
    t.is(error_bar.error_value.value, "Bar");
    t.throws(() => ResultOpaque.newFailingUnit());
    const error_struct = t.throws(() => ResultOpaque.newFailingStruct(109));
    t.is(error_struct.error_value.i, 109);
    const in_error = t.throws(() => ResultOpaque.newInErr(559));
    in_error.error_value.assertInteger(559);
    const in_enum_error = t.throws(() => ResultOpaque.newInEnumErr(881));
    in_enum_error.error_value.assertInteger(881);
});
