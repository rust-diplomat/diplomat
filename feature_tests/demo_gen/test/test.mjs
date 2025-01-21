import test from "ava";
import { MyStringDemo, CyclicStructADemo, CyclicStructCDemo, RenderInfo } from "diplomat-wasm-demo-gen-feature-tests";

test("My String", (t) => {
	t.is(MyStringDemo.stringTransform("a"), "");
})

test("Cyclic Parameters", (t) => {
	t.is(RenderInfo.termini["CyclicStructA.cyclicOut"].parameters[0].name, "CyclicStructA:A:Field");
	t.is(RenderInfo.termini["CyclicStructC.cyclicOut"].parameters[0].name, "CyclicStructC:A:A:Field");
	t.is(CyclicStructADemo.cyclicOut(10), "10");
	t.is(CyclicStructCDemo.cyclicOut(15), "15");
});

test("Variable Name Collisions", (t) => {
	// Make sure that the names for the parameters are the same so that we're assured some collisions.
	// As long as this test passes and the Javascript compiles, we should be good to go.
	t.is(RenderInfo.termini["CyclicStructA.doubleCyclicOut"].parameters[0].name, "CyclicStructA:A:Field");
	t.is(RenderInfo.termini["CyclicStructA.doubleCyclicOut"].parameters[1].name, "CyclicStructA:A:Field");

	t.is(CyclicStructADemo.doubleCyclicOut(10, 20), "10 20");
});

test("Getter", (t) => {
	t.is(CyclicStructADemo.getterOut(10), "10");
});