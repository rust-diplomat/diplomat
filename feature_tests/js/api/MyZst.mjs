// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



export class MyZst {
	
    /** Create `MyZst` from an object that contains all of `MyZst`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    static FromFields(structObj) {
        return new MyZst(structObj);
    }
    
    #internalConstructor(structObj) {
        if (typeof structObj !== "object") {
            throw new Error("MyZst's constructor takes an object of MyZst's fields.");
        }

    }
    
    constructor(structObj) {
        this.#internalConstructor(structObj);
    }

}