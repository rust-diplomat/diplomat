import { Float64Vec } from "./js/index.mjs"
export function toString() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].toString(...args.slice(1)) }).apply(
        null,
        [
            Float64Vec.newFromOwned.apply(
                null,
                [
                    terminusArgs[0]
                ]
            )
        ]
    );
}
