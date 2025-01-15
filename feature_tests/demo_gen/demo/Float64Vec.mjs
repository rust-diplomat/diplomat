import { Float64Vec } from "../../js/api/index.mjs"
export function toString(v) {
    return (function (...args) { return args[0].toString(...args.slice(1)) }).apply(
        null,
        [
            (function (...args) { return new Float64Vec(...args) } ).apply(
                null,
                [
                    v
                ]
            )
        ]
    );
}
