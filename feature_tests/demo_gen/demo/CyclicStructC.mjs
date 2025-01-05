import { CyclicStructA } from "../../js/api/index.mjs"
import { CyclicStructB } from "../../js/api/index.mjs"
import { CyclicStructC } from "../../js/api/index.mjs"
export function cyclicOut(field) {
    return (function (...args) { return args[0].cyclicOut(...args.slice(1)) }).apply(
        null,
        [
            (function (...args) {
                return CyclicStructC.FromFields({
                    a: args[0]});
            }).apply(
                null,
                [
                    (function (...args) {
                        return CyclicStructA.FromFields({
                            a: args[0]});
                    }).apply(
                        null,
                        [
                            (function (...args) {
                                return CyclicStructB.FromFields({
                                    field: args[0]});
                            }).apply(
                                null,
                                [
                                    field
                                ]
                            )
                        ]
                    )
                ]
            )
        ]
    );
}
