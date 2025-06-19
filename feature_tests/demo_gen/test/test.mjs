import test from "ava";
import { RenderInfo } from "diplomat-wasm-demo-gen-feature-tests";

test("My String", (t) => {
	t.is(RenderInfo.termini["MyString.stringTransform"]["func"]("a"), "");
})

test("Cyclic Parameters", (t) => {
	t.is(RenderInfo.termini["CyclicStructA.cyclicOut"].parameters[0].name, "self_a_field");
	t.is(RenderInfo.termini["CyclicStructC.cyclicOut"].parameters[0].name, "self_a_a_field");
	t.is(RenderInfo.termini["CyclicStructA.cyclicOut"]["func"](10), "10");
	t.is(RenderInfo.termini["CyclicStructC.cyclicOut"]["func"](15), "15");
});

test("Expression", (t) => {
	t.is(RenderInfo.termini["MyString.stringTransform"]["expr"]("'a'"), "somelib.MyString.stringTransform('a')");
})

test("Variable Name Collisions", (t) => {
	// Make sure that the names for the parameters are the same so that we're assured some collisions.
	// As long as this test passes and the Javascript compiles, we should be good to go.
	t.is(RenderInfo.termini["CyclicStructA.doubleCyclicOut"].parameters[0].name, "self_a_field");
	t.is(RenderInfo.termini["CyclicStructA.doubleCyclicOut"].parameters[1].name, "cyclicStructA_a_field");

	t.is(RenderInfo.termini["CyclicStructA.doubleCyclicOut"]["func"](10, 20), "10 20");
});

test("Getter", (t) => {
	t.is(RenderInfo.termini["CyclicStructA.getterOut"]["func"](10), "10");
});