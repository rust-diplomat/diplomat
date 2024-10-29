import { OptionString } from "../../js/api/index.mjs"
export function write(diplomatStr) {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].write(...args.slice(1)) }).apply(
        null,
        [
            OptionString.new_.apply(
                null,
                [
                    terminusArgs[0]
                ]
            )
        ]
    );
}
