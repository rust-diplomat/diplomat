// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"


export class Bar {
	
	#bEdge;
	
	#aEdge;
	
	
	foo() {
        const result = wasm.Bar_foo();
        return new Foo(result, bEdges, aEdges);;
    }
	
}