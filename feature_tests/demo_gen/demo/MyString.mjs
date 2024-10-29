import { MyString } from "../../js/api/index.mjs"
export function getStr(v) {
    return (function (...args) { return args[0].getStr }).apply(
        null,
        [
            MyString.new_.apply(
                null,
                [
                    v
                ]
            )
        ]
    );
}
export function stringTransform(foo) {
    return MyString.stringTransform.apply(
        null,
        [
            foo
        ]
    );
}
