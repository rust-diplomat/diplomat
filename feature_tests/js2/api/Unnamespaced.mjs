// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"


export class Unnamespaced {
	
	
	static make(e) {
        const result = wasm.namespace_Unnamespaced_make();
        return new Unnamespaced(result, []);;
    }
	
	useNamespaced(n) {
        wasm.namespace_Unnamespaced_use_namespaced();
        
    }
	
}