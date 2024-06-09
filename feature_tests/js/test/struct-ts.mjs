import test from 'ava';
import { MyEnum, MyStruct, Opaque, Utf16Wrap } from "diplomat-wasm-feature-tests";

test("Verify invariants of struct", t => {
    const s = MyStruct.new();
    t.is(s["a"], 17);
    t.is(s["b"], true);
    t.is(s["c"], 209);
    t.is(s["d"], 1234n);
    t.is(s["e"], 5991);
    t.is(s["f"], "餐");
    t.is(s["g"], MyEnum.B);
});

test("Check string conversions", t => {
    let input = "Hello 🗺";
    let str_wrap = Opaque.try_from_utf8(input);
    let utf8_wrap = Opaque.try_from_utf8(input);
    let utf16_wrap = Utf16Wrap.from_utf16(input);
    t.is(str_wrap.get_debug_str(), '"Hello 🗺"');
    t.is(utf8_wrap.get_debug_str(), '"Hello 🗺"');
    t.is(utf16_wrap.get_debug_str(), '[72, 101, 108, 108, 111, 32, 55357, 56826]');
});
