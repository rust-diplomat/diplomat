import { ICU4XFixedDecimal } from "../ICU4XFixedDecimal.mjs"

export function toString(v) {
	return ((...args) => { return this.toString(...args) }).call(
        ICU4XFixedDecimal.new_.call(
            null,
            arguments[0]
        )
    );
}
