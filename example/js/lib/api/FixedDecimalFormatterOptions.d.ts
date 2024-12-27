// generated by diplomat-tool
import type { FixedDecimalGroupingStrategy } from "./FixedDecimalGroupingStrategy"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

type FixedDecimalFormatterOptions_obj = {
    groupingStrategy: FixedDecimalGroupingStrategy;
    someOtherConfig: boolean;
};



export class FixedDecimalFormatterOptions {
	

    get groupingStrategy() : FixedDecimalGroupingStrategy;
    set groupingStrategy(value: FixedDecimalGroupingStrategy); 

    get someOtherConfig() : boolean;
    set someOtherConfig(value: boolean); 

    
    #internalConstructor(structObj : FixedDecimalFormatterOptions_obj);


    #defaultConstructor(): FixedDecimalFormatterOptions;

    constructor() {
        if (arguments[0] === diplomatRuntime.internalConstructor) {
            this.#internalConstructor(...arguments.slice(1));
        } else {
            this.#defaultConstructor(...arguments);
        }
    }
}