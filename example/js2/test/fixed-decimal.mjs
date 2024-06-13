import test from "ava";

import { ICU4XFixedDecimalFormatterDemo } from "demo/demo"; 

test("Format write exists.", t => {
	t.notThrows(() => {ICU4XFixedDecimalFormatterDemo.formatWrite()});
});

test("Format write returns a formatted decimal.", t => {
	t.is("1,000", ICU4XFixedDecimalFormatterDemo.formatWrite(1000));
});