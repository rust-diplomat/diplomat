import test from 'ava';
import { Foo } from "diplomat-wasm-js-feature-tests";

test("Foo Static", (t) => {
	let f = Foo.newStatic("This is a test string.");
	t.not(f.ffiValue, null);

	let returning = f.asReturning();

	t.is(returning.bytes, "This is a test string.");

	let b = f.bar.foo.asReturning().bytes;
	t.is(b, "This is a test string.");
});

test("Foo", (t) => {
	let f = Foo.new_("This is a test string.");
	t.not(f.ffiValue, null);

	let returning = f.asReturning();
	t.is(returning.bytes, "This is a test string.");

	let b = f.bar.foo.asReturning().bytes;
	t.is(b, "This is a test string.");
});