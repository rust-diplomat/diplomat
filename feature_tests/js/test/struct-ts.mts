import test from 'ava';
import { CyclicStructA, MyEnum, MyStruct } from "diplomat-wasm-js-feature-tests";

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

test("Nested Struct Construction Parameters", t => {
    const nested = new CyclicStructA({
        a: {
            field: 10
        }
    });
    t.is(nested.cyclicOut(), "10");
});