import test from 'ava';
import { MyString } from "diplomat-wasm-js-feature-tests";

test("MyString functionality", (t) => {
	let str = MyString.new_("This is a test value.");
	t.is(str.str, "This is a test value.");
});