import { ICU4XDataProvider } from "../ICU4XDataProvider.mjs"
import { ICU4XFixedDecimal } from "../ICU4XFixedDecimal.mjs"
import { ICU4XFixedDecimalFormatter } from "../ICU4XFixedDecimalFormatter.mjs"
import { ICU4XFixedDecimalFormatterOptions } from "../ICU4XFixedDecimalFormatterOptions.mjs"
import { ICU4XLocale } from "../ICU4XLocale.mjs"

export function formatWrite(name, grouping_strategy, some_other_config, v) {
	return ((...args) => { return this.formatWrite(...args) }).call(
        ICU4XFixedDecimalFormatter.tryNew.call(
            null,
            ICU4XLocale.new_.call(
                null,
                arguments[0]
            ),
            ICU4XDataProvider.newStatic.call(
                null
            ),
            ((...args) => {
            	let out = new ICU4XFixedDecimalFormatterOptions();
            	
            	out.groupingStrategy = args[0];
            	
            	out.someOtherConfig = args[1];
            	
            	return out;
            }).call(
                arguments[1],
                arguments[2]
            )
        ),
        ICU4XFixedDecimal.new_.call(
            null,
            arguments[3]
        )
    );
}
