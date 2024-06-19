import test from "ava";

import { ICU4XFixedDecimalDemo, ICU4XFixedDecimalFormatterDemo } from "demo/demo"; 
import { ICU4XFixedDecimalGroupingStrategy } from "../api/ICU4XFixedDecimalGroupingStrategy.mjs";

test("Format write returns a formatted decimal.", t => {
	t.is("1,000", ICU4XFixedDecimalFormatterDemo.formatWrite("en", ICU4XFixedDecimalGroupingStrategy.Auto, false, 1000));
});

test("To string returns the proper string.", t => {
	t.is(1000, ICU4XFixedDecimalDemo.toString(1000));
});