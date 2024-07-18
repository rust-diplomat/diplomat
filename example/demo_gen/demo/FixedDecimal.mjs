import { FixedDecimal } from "../FixedDecimal.mjs"
export function toString() {
	var terminusArgs = arguments;
	return (function (...args) { return this.toString(...args) }).apply(
        FixedDecimal.new_.apply(
        null,
        [
            terminusArgs[0]
        ]
    ),
        [
        ]
    );
}
