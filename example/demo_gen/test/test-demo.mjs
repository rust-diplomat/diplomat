import test from "ava";
import { FixedDecimalDemo, FixedDecimalFormatterDemo } from "mini-icu4x-demo";
import { FixedDecimalGroupingStrategy } from "mini-icu4x";


test("Test FixedDecimal", (t) => {
	t.is(FixedDecimalDemo.toString(100), "100");
});

test("Test FixedDecimalFormatter", (t) => {
	t.is(FixedDecimalFormatterDemo.formatWrite("en", FixedDecimalGroupingStrategy.Always, false, 1000), "1,000");
});

test("Custom Function", (t) => {
	t.is(FixedDecimalDemo.multiplyPow10(3), "1000");
});