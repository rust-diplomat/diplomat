import { ICU4XFixedDecimal } from "../ICU4XFixedDecimal.mjs"

export function toString(v) {
	var terminusArgs = arguments;
	return (function (...args) { return this.toString(...args) }).apply(
        ICU4XFixedDecimal.new_.apply(
        null,
        [
            terminusArgs[0]
        ]
    ),
        [
        ]
    );
}
