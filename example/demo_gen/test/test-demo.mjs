import test from "ava";
import { FixedDecimalDemo, FixedDecimalFormatterDemo, RenderInfo } from "mini-icu4x-demo";
import { FixedDecimalGroupingStrategy } from "mini-icu4x";


test("Test FixedDecimal", (t) => {
	t.is(FixedDecimalDemo.toString(100), "100");
});

test("Test FixedDecimalFormatter", (t) => {
	t.is(FixedDecimalFormatterDemo.formatWrite("en", FixedDecimalGroupingStrategy.Always, false, 1000), "1,000");
});

test("Custom Function", (t) => {
	t.is(RenderInfo.termini["FixedDecimal.multiplyPow10"].func(3), "10000");
});

test("Variable Names", (t) => {
	// Can't exactly check variable names without reading the file, but RenderInfo re-uses the same info, so we check that instead.
	t.is(RenderInfo.termini["FixedDecimalFormatter.formatWrite"].parameters[0].name, "FixedDecimalFormatter:Locale:Name");
});