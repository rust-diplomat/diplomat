import { ICU4XDataProvider } from "../ICU4XDataProvider.mjs"
import { ICU4XFixedDecimal } from "../ICU4XFixedDecimal.mjs"
import { ICU4XFixedDecimalFormatter } from "../ICU4XFixedDecimalFormatter.mjs"
import { ICU4XFixedDecimalFormatterOptions } from "../ICU4XFixedDecimalFormatterOptions.mjs"
import { ICU4XLocale } from "../ICU4XLocale.mjs"

export function formatWrite(name, grouping_strategy, some_other_config, v) {
	var terminusArgs = arguments;
	return (function (...args) { return this.formatWrite(...args) }).apply(
        ICU4XFixedDecimalFormatter.tryNew.apply(
        null,
        [
            ICU4XLocale.new_.apply(
                null,
                [
                    terminusArgs[0]
                ]
            ),
            ICU4XDataProvider.newStatic.apply(
                null,
                [
                ]
            ),
            (function (...args) {
            	let out = new ICU4XFixedDecimalFormatterOptions();
            	
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
        [
            ICU4XFixedDecimal.new_.apply(
                null,
                [
                    terminusArgs[3]
                ]
            )
        ]
    );
}
