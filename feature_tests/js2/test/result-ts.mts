import test from 'ava';
import { FFIError } from "diplomat-wasm-feature-tests";
import { ErrorEnum } from "diplomat-wasm-feature-tests";
import { ErrorStruct } from "diplomat-wasm-feature-tests";
import { ResultOpaque } from "diplomat-wasm-feature-tests"

test("Verify result methods", t => {
    const s = ResultOpaque.new(5);
    s.assert_integer(5);

    const error_foo = t.throws(() => ResultOpaque.new_failing_foo()) as unknown as FFIError<ErrorEnum>;
    t.is(error_foo.error_value, "Foo");
    const error_bar = t.throws(() => ResultOpaque.new_failing_bar()) as unknown as FFIError<ErrorEnum>;
    t.is(error_bar.error_value, "Bar");
    t.throws(() => ResultOpaque.new_failing_unit());
    const error_struct = t.throws(() => ResultOpaque.new_failing_struct(109)) as unknown as FFIError<ErrorStruct>;
    t.is(error_struct.error_value.i, 109);

    const in_error = t.throws(() => ResultOpaque.new_in_err(559)) as unknown as FFIError<ResultOpaque>;
    in_error.error_value.assert_integer(559);
    const in_enum_error = t.throws(() => ResultOpaque.new_in_enum_err(881)) as unknown as FFIError<ResultOpaque>;
    in_enum_error.error_value.assert_integer(881);
});
