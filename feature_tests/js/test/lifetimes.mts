import test from 'ava';
import { Foo } from "diplomat-wasm-js-feature-tests";

test("Foo", (t) => {
	let f = Foo.new_("This is a test string.");
	t.not(f.ffiValue, null);

	let returning = f.asReturning();
	t.is(returning.bytes.toString(), "This is a test string.");

	let b = f.bar.foo.asReturning().bytes;
	t.is(b.toString(), "This is a test string.");
});