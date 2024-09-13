import test from "ava";
import { MyStringDemo } from "diplomat-wasm-demo-gen-feature-tests";

test("My String", (t) => {
	t.is(MyStringDemo.stringTransform("a"), "");
})