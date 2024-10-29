import { MyString } from "../../js/api/index.mjs"
export function getStr(v) {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].getStr }).apply(
        null,
        [
            MyString.new_.apply(
                null,
                [
                    terminusArgs[0]
                ]
            )
        ]
    );
}
export function stringTransform(foo) {
    var terminusArgs = arguments;
    return MyString.stringTransform.apply(
        null,
        [
            terminusArgs[0]
        ]
    );
}
