import test from "ava";

import { FixedDecimalDemo, FixedDecimalFormatterDemo } from "demo/demo"; 
import { FixedDecimalGroupingStrategy } from "demo";

test("Format write returns a formatted decimal.", t => {
	t.is("1,000", FixedDecimalFormatterDemo.formatWrite("en", FixedDecimalGroupingStrategy.Auto, false, 1000));
});

test("toString returns the proper string.", t => {
	t.is("1000", FixedDecimalDemo.toString(1000));
});