import { FixedDecimal } from "./js/FixedDecimal.mjs"
export function toString() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].toString(...args.slice(1)) }).apply(
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
