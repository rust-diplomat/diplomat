import { ICU4XDataProvider } from "./ICU4XDataProvider.mjs"
import { ICU4XFixedDecimal } from "./ICU4XFixedDecimal.mjs"
import { ICU4XFixedDecimalFormatter } from "./ICU4XFixedDecimalFormatter.mjs"
import { ICU4XLocale } from "./ICU4XLocale.mjs"

export function formatWrite(name, v) {
	return ICU4XFixedDecimalFormatter.formatWrite.call(
	
	ICU4XFixedDecimalFormatter.tryNew.call(
	
	null,
	
	ICU4XLocale.new_.call(
	
	null,
	
	name
	
),
	
	ICU4XDataProvider.newStatic.call(
	
	null
	
)
	
),
	
	ICU4XFixedDecimal.new_.call(
	
	null,
	
	v
	
)
	
);
}
