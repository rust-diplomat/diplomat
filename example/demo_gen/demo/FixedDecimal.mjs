import { FixedDecimal } from "../../js/lib/api/index.mjs"
import { multiplyPow10 } from "./a.mjs"
export function multiplyPow10() {
    var terminusArgs = arguments;
    return multiplyPow10(...terminusArgs);;
}
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
