// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"


export class OptionOpaque {
	
	
	static new(i) {
        const result = wasm.OptionOpaque_new();
        return (result === 0) ? undefined : new OptionOpaque(result, []);;
    }
	
	static newNone() {
        const result = wasm.OptionOpaque_new_none();
        return (result === 0) ? undefined : new OptionOpaque(result, []);;
    }
	
	static returns() {
        const result = wasm.OptionOpaque_returns();
        if (!result.isOk) {
        return null
    }
     return OptionStruct // TODO;
    }
	
	static newStruct() {
        const result = wasm.OptionOpaque_new_struct();
        return OptionStruct // TODO;
    }
	
	static newStructNones() {
        const result = wasm.OptionOpaque_new_struct_nones();
        return OptionStruct // TODO;
    }
	
	assertInteger(i) {
        wasm.OptionOpaque_assert_integer();
        
    }
	
	static optionOpaqueArgument(arg) {
        const result = wasm.OptionOpaque_option_opaque_argument();
        return result;
    }
	
}