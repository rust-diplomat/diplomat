import test from 'ava';

import { OptionOpaque } from "diplomat-wasm-js2-feature-tests";

test("Verify option methods", t => {
    const o = OptionOpaque.new(5);
    o.assert_integer(5);

    const on = OptionOpaque.new_none();
    t.assert(!on);

    const s = OptionOpaque.new_struct();

    s.a.assert_integer(101);
    s.b.assert_char('餐');
    t.is(s.c, 904);
    s.d.assert_integer(926535);

    const sn = OptionOpaque.new_struct_nones();
    t.assert(!sn.a);
    t.assert(!sn.b);
    t.is(sn.c, 908);
    t.assert(!sn.d);
});
