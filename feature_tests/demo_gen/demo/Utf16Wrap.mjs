import { Utf16Wrap } from "./js/index.mjs"
export function getDebugStr() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].getDebugStr(...args.slice(1)) }).apply(
        null,
        [
            Utf16Wrap.fromUtf16.apply(
                null,
                [
                    terminusArgs[0]
                ]
            )
        ]
    );
}
