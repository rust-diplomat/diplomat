import test from 'ava';
import { MyEnum, MyStruct, CyclicStructB, CyclicStructC, ScalarPairWithPadding, BigStructWithStuff, DefaultEnum, StructWithSlices } from "diplomat-wasm-js-feature-tests";

test("Verify invariants of struct", t => {
    const s = new MyStruct();
    t.is(s.a, 17);
    t.is(s.b, true);
    t.is(s.c, 209);
    t.is(s.d, 1234n);
    t.is(s.e, 5991);
    t.is(s.f, '餐'.codePointAt(0));
    t.is(s.g, MyEnum.B);
    t.is(s.intoA(), 17);
});

test("Test struct creation", t => {
    const s = MyStruct.fromFields({
        a: 17,
        b: true,
        c: 209,
        d: 1234n,
        e: 5991,
        f: '餐'.codePointAt(0),
        g: MyEnum.B
    });
    t.is(s.intoA(), 17);
});

test("Function Takes Nested Struct Parameters", t => {
    const nested = CyclicStructC.takesNestedParameters({
        a: {
            a: {
                field: 10
            }
        }
    });
    t.is(nested.cyclicOut(), "10");
});

test("Nested Struct Construction", t => {
    const nested = new CyclicStructC({
        a: {
            a: {
                field: 10
            }
        }
    });
    t.is(nested.cyclicOut(), "10");
    // Test that CyclicStructA is constructed from our object:
    t.is(nested.a.cyclicOut(), "10");
});

test("Nested Struct with pre-built Object", t => {
    const existing = new CyclicStructB({ field: 15 });
    const nested = new CyclicStructC({
        a: {
            a: existing
        }
    });
    t.is(nested.cyclicOut(), "15");
});

test("Test struct layout: scalar pair layout", t => {
    const s = new ScalarPairWithPadding({
        first: 122,
        second: 414
    });
    t.notThrows(() => { s.assertValue() });
});

test("Test struct layout: complex struct with multiple padding types and contained scalar pair", t => {
    const s = new BigStructWithStuff({
        first: 101,
        second: 505,
        third: 9345,
        fourth: new ScalarPairWithPadding({first: 122, second: 414}),
        fifth: 99
    });
    t.notThrows(() => { s.assertValue(853) });
});

test("Function Returning Nested Struct of One Primitive", t => {
    const a = CyclicStructB.getA();
    t.is(a.cyclicOut(), "0");
});

test("Function De-Referencing Nested Struct of One Primitive", t => {
    const a = CyclicStructB.getAOption();
    t.is(a.cyclicOut(), "0");
});

test("Verify Enum Construction", t => {
    t.is(new DefaultEnum(), DefaultEnum.A);

	t.is(DefaultEnum.fromValue("B"), DefaultEnum.B);
});

test("Passing struct self with slice", t => {
    let s = new StructWithSlices({
        first: "testing",
        second: []
    });
    t.is(s.returnLast(), "g");
});