import wasm from "./wasm.mjs"
const ICU4XFixedDecimal_destroy_registry = new FinalizationRegistry(underlying => {
wasm.ICU4XFixedDecimal_destroy(underlying);
});
export class ICU4XFixedDecimal {
constructor(underlying) {
this.underlying = underlying;
}
static new(v) {
return (() => {
const out = new ICU4XFixedDecimal(wasm.ICU4XFixedDecimal_new(v));
ICU4XFixedDecimal_destroy_registry.register(out, out.underlying);
return out;
})();
}
multiply_pow10(power) {
wasm.ICU4XFixedDecimal_multiply_pow10(this.underlying, power);
}
digit_at(magnitude) {
return wasm.ICU4XFixedDecimal_digit_at(this.underlying, magnitude);
}
}