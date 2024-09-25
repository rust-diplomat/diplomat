import lib from "./index.mjs";
import { FixedDecimal } from "../../js/lib/api/index.mjs"
import { multiplyPow10 as multiplyPow10Custom } from "./a.mjs";
export function multiplyPow10() {
    var terminusArgs = arguments;
    return multiplyPow10Custom(lib, ...terminusArgs);
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
