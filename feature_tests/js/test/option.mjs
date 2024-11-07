import test from 'ava';

import { OptionOpaque, OptionInputStruct, OptionEnum } from "diplomat-wasm-js-feature-tests";

test("Verify option methods", t => {
    const o = OptionOpaque.new_(5);
    o.assertInteger(5);

    const on = OptionOpaque.newNone();
    t.assert(!on);

    const s = OptionOpaque.newStruct();

    s.a.assertInteger(101);
    s.b.assertChar('餐'.codePointAt(0));
    t.is(s.c, 904);
    s.d.assertInteger(926535);

    const sn = OptionOpaque.newStructNones();
    t.assert(!sn.a);
    t.assert(!sn.b);
    t.is(sn.c, 908);
    t.assert(!sn.d);
});

test("DiplomatOption tests", t => {
    let maybeU8 = OptionOpaque.acceptsOptionU8(null);
    t.assert(maybeU8 === null);
    maybeU8 = OptionOpaque.acceptsOptionU8(47);
    t.is(maybeU8, 47);

    let maybeStruct = OptionOpaque.acceptsOptionInputStruct(null);
    t.assert(maybeStruct === null);
    maybeStruct = OptionOpaque.acceptsOptionInputStruct(new OptionInputStruct({a: 7, c: OptionEnum.Bar}));
    t.is(maybeStruct.a, 7);
    t.assert(maybeStruct.b === null);
    t.is(maybeStruct.c.value, OptionEnum.Bar.value);


    let struct = OptionOpaque.returnsOptionInputStruct();
    t.is(struct.a, 6);
    t.assert(struct.b === null);
    t.is(struct.c.value, OptionEnum.Bar.value);

});
