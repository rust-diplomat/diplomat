import test from 'ava';
import { CyclicStructB, CyclicStructC, MyEnum, MyStruct } from "diplomat-wasm-js-feature-tests";
test("Verify invariants of struct", t => {
    const s = MyStruct.new_();
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
    const s = new MyStruct({
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
            },
            b: 0
        }
    });
    t.is(nested.cyclicOut(), "10");
});
test("Nested Struct Construction", t => {
    const nested = new CyclicStructC({
        a: {
            a: {
                field: 10
            },
            b: 0
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
            a: existing,
            b: 0
        }
    });
    t.is(nested.cyclicOut(), "15");
});
