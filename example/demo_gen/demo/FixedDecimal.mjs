import { FixedDecimal } from "../../js/lib/api/index.mjs"
export function toString(v) {
    return (function (...args) { return args[0].toString(...args.slice(1)) }).apply(
        null,
        [
            (function (...args) { return new FixedDecimal(...args) } ).apply(
                null,
                [
                    v
                ]
            )
        ]
    );
}
