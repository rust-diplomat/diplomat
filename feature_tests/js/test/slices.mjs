import test from 'ava';
import { MyString, Float64Vec} from "diplomat-wasm-js-feature-tests";

test("MyString functionality", (t) => {
    let str = MyString.new_("This is a test value.");
    t.is(str.str, "This is a test value.");
});

test("String List", (t) => {
	let str = MyString.newFromFirst(["This", "is", "a", "test."]);
	t.is(str.str, "This");
});
