import { DataProvider } from "../../js/lib/api/index.mjs"
import { FixedDecimal } from "../../js/lib/api/index.mjs"
import { FixedDecimalFormatter } from "../../js/lib/api/index.mjs"
import { FixedDecimalFormatterOptions } from "../../js/lib/api/index.mjs"
import { Locale } from "../../js/lib/api/index.mjs"
export function formatWrite(selfLocaleName, selfOptionsGroupingStrategy, selfOptionsSomeOtherConfig, valueV) {
    
    let selfLocale = new Locale(selfLocaleName);
    
    let selfProvider = DataProvider.newStatic();
    
    let selfOptions = FixedDecimalFormatterOptions.fromFields({
        groupingStrategy: selfOptionsGroupingStrategy,
        someOtherConfig: selfOptionsSomeOtherConfig
    });
    
    let self = FixedDecimalFormatter.tryNew(selfLocale,selfProvider,selfOptions);
    
    let value = new FixedDecimal(valueV);
    
    let out = self.formatWrite(value);
    

    return out;
}
