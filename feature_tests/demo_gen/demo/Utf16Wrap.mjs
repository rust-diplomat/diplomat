import { Utf16Wrap } from "../../js/api/index.mjs"
export function getDebugStr(input) {
    return (function (...args) { return args[0].getDebugStr(...args.slice(1)) }).apply(
        null,
        [
            Utf16Wrap.fromUtf16.apply(
                null,
                [
                    input
                ]
            )
        ]
    );
}
