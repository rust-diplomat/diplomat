import { Utf16Wrap } from "../../js/api/index.mjs"
export function getDebugStr(input) {
    return (function (...args) { return args[0].getDebugStr(...args.slice(1)) }).apply(
        null,
        [
            (function (...args) { return new Utf16Wrap(...args) } ).apply(
                null,
                [
                    input
                ]
            )
        ]
    );
}
