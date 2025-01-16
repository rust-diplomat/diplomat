import { DataProvider } from "../../js/lib/api/index.mjs"
import { FixedDecimal } from "../../js/lib/api/index.mjs"
import { FixedDecimalFormatter } from "../../js/lib/api/index.mjs"
import { FixedDecimalFormatterOptions } from "../../js/lib/api/index.mjs"
import { Locale } from "../../js/lib/api/index.mjs"
export function formatWrite(name, grouping_strategy, some_other_config, v) {
    
    let locale = new Locale(name);
    
    let provider = DataProvider.newStatic();
    
    let options = FixedDecimalFormatterOptions.fromFields({
        groupingStrategy: grouping_strategy,
        someOtherConfig: some_other_config
    });
    
    let self = FixedDecimalFormatter.tryNew(locale,provider,options);
    
    let value = new FixedDecimal(v);
    
    let out = self.formatWrite(value);
    

    return out;
}
