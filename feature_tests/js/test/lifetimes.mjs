import test from 'ava';
import { BorrowedFields, BorrowedFieldsReturning, BorrowedFieldsWithBounds, Foo, Bar } from "diplomat-wasm-js-feature-tests";

test("Foo", (t) => {
	let f = Foo.newStatic("This is a test string.");
	t.not(f.ffiValue, null);

	let returning = f.asReturning();
	t.is(returning.bytes, "This is a test string.");
});