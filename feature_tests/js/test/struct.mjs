import test from 'ava';
import { MyEnum, MyStruct, ScalarPairWithPadding, BigStructWithStuff } from "diplomat-wasm-js-feature-tests";

test("Verify invariants of struct", t => {
    const s = MyStruct.new_("hello");
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

test("Test struct layout: scalar pair layout", t => {
    const s = new ScalarPairWithPadding({
        first: 122,
        second: 414
    });
    s.assertValue();
    t.is(true, true); // Ava doesn't like tests without assertions
});

test("Test struct layout: complex struct with multiple padding types and contained scalar pair", t => {
    const s = new BigStructWithStuff({
        first: 101, 
        second: 505, 
        third: 9345,
        fourth: new ScalarPairWithPadding({first: 122, second: 414}),
        fifth: 99
    });
    s.assertValue(853);
    t.is(true, true); // Ava doesn't like tests without assertions
});
