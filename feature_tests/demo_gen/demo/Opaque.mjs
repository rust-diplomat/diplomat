import { Opaque } from "./js/index.mjs"
export function getDebugStr() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].getDebugStr(...args.slice(1)) }).apply(
        null,
        [
            Opaque.new_.apply(
                null,
                [
                ]
            )
        ]
    );
}
