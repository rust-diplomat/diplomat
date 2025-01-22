import { DataProvider } from "../../js/lib/api/index.mjs"
import { FixedDecimal } from "../../js/lib/api/index.mjs"
import { FixedDecimalFormatter } from "../../js/lib/api/index.mjs"
import { FixedDecimalFormatterOptions } from "../../js/lib/api/index.mjs"
import { Locale } from "../../js/lib/api/index.mjs"
export function formatWrite(fixedDecimalFormatterLocaleName, fixedDecimalFormatterOptionsGroupingStrategy, fixedDecimalFormatterOptionsSomeOtherConfig, valueV) {
    
    let fixedDecimalFormatterLocale = new Locale(fixedDecimalFormatterLocaleName);
    
    let fixedDecimalFormatterProvider = DataProvider.newStatic();
    
    let fixedDecimalFormatterOptions = FixedDecimalFormatterOptions.fromFields({
        groupingStrategy: fixedDecimalFormatterOptionsGroupingStrategy,
        someOtherConfig: fixedDecimalFormatterOptionsSomeOtherConfig
    });
    
    let fixedDecimalFormatter = FixedDecimalFormatter.tryNew(fixedDecimalFormatterLocale,fixedDecimalFormatterProvider,fixedDecimalFormatterOptions);
    
    let value = new FixedDecimal(valueV);
    
    let out = fixedDecimalFormatter.formatWrite(value);
    

    return out;
}
