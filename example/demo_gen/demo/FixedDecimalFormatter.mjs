import { DataProvider } from "../../js/lib/api/index.mjs"
import { FixedDecimal } from "../../js/lib/api/index.mjs"
import { FixedDecimalFormatter } from "../../js/lib/api/index.mjs"
import { FixedDecimalFormatterOptions } from "../../js/lib/api/index.mjs"
import { Locale } from "../../js/lib/api/index.mjs"
export function formatWrite(fixedDecimalFormatterLocaleName, fixedDecimalFormatterOptionsFixedDecimalFormatterGroupingStrategy, fixedDecimalFormatterOptionsFixedDecimalFormatterSomeOtherConfig, valueV) {
    
    let locale = new Locale(fixedDecimalFormatterLocaleName);
    
    let provider = DataProvider.newStatic();
    
    let options = FixedDecimalFormatterOptions.fromFields({
        groupingStrategy: fixedDecimalFormatterOptionsFixedDecimalFormatterGroupingStrategy,
        someOtherConfig: fixedDecimalFormatterOptionsFixedDecimalFormatterSomeOtherConfig
    });
    
    let fixedDecimalFormatter = FixedDecimalFormatter.tryNew(locale,provider,options);
    
    let value = new FixedDecimal(valueV);
    
    let out = fixedDecimalFormatter.formatWrite(value);
    

    return out;
}
