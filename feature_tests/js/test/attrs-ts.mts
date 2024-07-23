import test from "ava";

import { RenamedMyIterable } from "diplomat-wasm-js-feature-tests";

test("Verify Iterables and Iterators", t => {
	let iterable = RenamedMyIterable.new_([10, 20, 30, 40, 50]);

	let start = 10;
	for (let i of iterable) {
		t.is(i, start);
		start += 10;
	}
});