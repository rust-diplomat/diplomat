import { DataProvider } from "../DataProvider.mjs"
import { FixedDecimal } from "../FixedDecimal.mjs"
import { FixedDecimalFormatter } from "../FixedDecimalFormatter.mjs"
import { FixedDecimalFormatterOptions } from "../FixedDecimalFormatterOptions.mjs"
import { Locale } from "../Locale.mjs"
export function formatWrite() {
	var terminusArgs = arguments;
	return (function (...args) { return args[0].formatWrite(...args.slice(1)) }).apply(
        null,
        [
            FixedDecimalFormatter.tryNew.apply(
                null,
                [
                    Locale.new_.apply(
                        null,
                        [
                            terminusArgs[0]
                        ]
                    ),
                    DataProvider.newStatic.apply(
                        null,
                        [
                        ]
                    ),
                    (function (...args) {
                    	let out = new FixedDecimalFormatterOptions();
                    	
                    	out.groupingStrategy = args[0];
                    	
                    	out.someOtherConfig = args[1];
                    	
                    	return out;
                    }).apply(
                        null,
                        [
                            terminusArgs[1],
                            terminusArgs[2]
                        ]
                    )
                ]
            ),
            FixedDecimal.new_.apply(
                null,
                [
                    terminusArgs[3]
                ]
            )
        ]
    );
}
