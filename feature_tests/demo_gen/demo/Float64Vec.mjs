import { Float64Vec } from "../../js/api/index.mjs"
export function toString(v) {
    return (function (...args) { return args[0].toString(...args.slice(1)) }).apply(
        null,
        [
            Float64Vec.newFromOwned.apply(
                null,
                [
                    v
                ]
            )
        ]
    );
}
