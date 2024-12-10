import { MyOpaqueEnum } from "../../js/api/index.mjs"
export function toString() {
    return (function (...args) { return args[0].toString(...args.slice(1)) }).apply(
        null,
        [
            MyOpaqueEnum.new_.apply(
                null,
                [
                ]
            )
        ]
    );
}
