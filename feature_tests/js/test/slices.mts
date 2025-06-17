import test from "ava";
import { MyString, Float64Vec } from "diplomat-wasm-js-feature-tests";

test("MyString functionality", (t) => {
  let str = new MyString("This is a test value.");
  t.is(str.str, "This is a test value.");
});

test("String List", (t) => {
  let str = MyString.newFromFirst(["This", "is", "a", "test."]);
  t.is(str.str, "This");
});

test("MyString borrow", (t) => {
  let str = new MyString("This is a test.");
  t.is(str.borrow(), "This is a test.");
});

test("Float64Vec", (t) => {
  let input = [1, 2, 3, 4, 5];
  let data = Float64Vec.newIsize(input);
  t.deepEqual(data.borrow(), input);
});

test("String Owned", async (t) => {
  let s = MyString.newOwned("This is a test.");
  t.is(s.borrow(), "This is a test.");
});