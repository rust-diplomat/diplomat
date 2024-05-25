// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"



const Float64Vec_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.Float64Vec_destroy(ptr);
});

export class Float64Vec {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        if (this.#selfEdge.length === 0) {
            Float64Vec_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }


    static newBool(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.slice(wasm, v, "bool");
        const result = wasm.Float64Vec_new_bool(vSlice.ptr, vSlice.size);
    
        vSlice.free();
        
        return new Float64Vec(result, []);
    }

    static newI16(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.slice(wasm, v, "i16");
        const result = wasm.Float64Vec_new_i16(vSlice.ptr, vSlice.size);
    
        vSlice.free();
        
        return new Float64Vec(result, []);
    }

    static newU16(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.slice(wasm, v, "u16");
        const result = wasm.Float64Vec_new_u16(vSlice.ptr, vSlice.size);
    
        vSlice.free();
        
        return new Float64Vec(result, []);
    }

    static newIsize(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.slice(wasm, v, "isize");
        const result = wasm.Float64Vec_new_isize(vSlice.ptr, vSlice.size);
    
        vSlice.free();
        
        return new Float64Vec(result, []);
    }

    static newUsize(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.slice(wasm, v, "usize");
        const result = wasm.Float64Vec_new_usize(vSlice.ptr, vSlice.size);
    
        vSlice.free();
        
        return new Float64Vec(result, []);
    }

    static newF64BeBytes(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.slice(wasm, v, "u8");
        const result = wasm.Float64Vec_new_f64_be_bytes(vSlice.ptr, vSlice.size);
    
        vSlice.free();
        
        return new Float64Vec(result, []);
    }

    static newFromOwned(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.slice(wasm, v, "f64");
        const result = wasm.Float64Vec_new_from_owned(vSlice.ptr, vSlice.size);
    
        return new Float64Vec(result, []);
    }

    get asBoxedSlice() {
        const result = wasm.Float64Vec_as_boxed_slice(this.ffiValue);
    
        return result // TODO: Slice c_to_js;
    }

    get asSlice() {
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];const result = wasm.Float64Vec_as_slice(this.ffiValue);
    
        return result(aEdges) // TODO: Slice c_to_js;
    }

    fillSlice(v) {
        
        const vSlice = diplomatRuntime.DiplomatBuf.slice(wasm, v, "f64");
        wasm.Float64Vec_fill_slice(this.ffiValue, vSlice.ptr, vSlice.size);
    
        vSlice.free();
        
    }

    setValue(newSlice) {
        
        const newSliceSlice = diplomatRuntime.DiplomatBuf.slice(wasm, newSlice, "f64");
        wasm.Float64Vec_set_value(this.ffiValue, newSliceSlice.ptr, newSliceSlice.size);
    
        newSliceSlice.free();
        
    }

    toString() {
        wasm.Float64Vec_to_string(this.ffiValue);
    
        return writeable;
    }

    borrow() {
        
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];const result = wasm.Float64Vec_borrow(this.ffiValue);
    
        return result(aEdges) // TODO: Slice c_to_js;
    }

    get(i) {
        const result = wasm.Float64Vec_get(this.ffiValue, i);
    
        if (!result.isOk) {
            return null
        }
         return result.union.ok;
    }

    

}