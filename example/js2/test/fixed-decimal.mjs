import test from "ava";

import { ICU4XFixedDecimalFormatterDemo } from "demo/demo"; 

test("Format write exists.", t => {
	t.notThrows(() => {ICU4XFixedDecimalFormatterDemo.formatWrite()});
});