import { Opaque } from "../../js/api/index.mjs"
export function getDebugStr() {
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
