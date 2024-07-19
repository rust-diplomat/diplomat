import { FixedDecimal } from "../FixedDecimal.mjs"
export function toString() {
	var terminusArgs = arguments;
	return (function (...args) { return args[0].toString(...args) }).apply(
        null,
        [
            FixedDecimal.new_.apply(
                null,
                [
                    terminusArgs[0]
                ]
            )
        ]
    );
}
