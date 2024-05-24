package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface Float64VecLib: Library {
    fun Float64Vec_destroy(handle: Pointer)
    fun Float64Vec_new_bool(v: Slice): Pointer
    fun Float64Vec_new_i16(v: Slice): Pointer
    fun Float64Vec_new_u16(v: Slice): Pointer
    fun Float64Vec_new_isize(v: Slice): Pointer
    fun Float64Vec_new_usize(v: Slice): Pointer
    fun Float64Vec_new_f64_be_bytes(v: Slice): Pointer
    fun Float64Vec_new_from_owned(v: Slice): Pointer
    fun Float64Vec_as_boxed_slice(handle: Pointer): Slice
    fun Float64Vec_as_slice(handle: Pointer): Slice
    fun Float64Vec_fill_slice(handle: Pointer, v: Slice): Unit
    fun Float64Vec_set_value(handle: Pointer, newSlice: Slice): Unit
    fun Float64Vec_to_string(handle: Pointer, writeable: Pointer): Unit
    fun Float64Vec_borrow(handle: Pointer): Slice
    fun Float64Vec_get(handle: Pointer, i: Long): Double?
}

class Float64Vec internal constructor (
    internal val handle: Pointer,

    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>) {

    internal class Float64VecCleaner(val handle: Pointer, val lib: Float64VecLib) : Runnable {
        override fun run() {
            lib.Float64Vec_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<Float64VecLib> = Float64VecLib::class.java
        internal val lib: Float64VecLib = Native.load("somelib", libClass)
        fun newBool(v: BooleanArray): Float64Vec {
            val (vMem, vSlice) = PrimitiveArrayTools.native(v)
            
            val returnVal = lib.Float64Vec_new_bool(vSlice);
        
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vMem.close()
            return returnOpaque
        
        }
        fun newI16(v: ShortArray): Float64Vec {
            val (vMem, vSlice) = PrimitiveArrayTools.native(v)
            
            val returnVal = lib.Float64Vec_new_i16(vSlice);
        
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vMem.close()
            return returnOpaque
        
        }
        fun newU16(v: UShortArray): Float64Vec {
            val (vMem, vSlice) = PrimitiveArrayTools.native(v)
            
            val returnVal = lib.Float64Vec_new_u16(vSlice);
        
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vMem.close()
            return returnOpaque
        
        }
        fun newIsize(v: LongArray): Float64Vec {
            val (vMem, vSlice) = PrimitiveArrayTools.native(v)
            
            val returnVal = lib.Float64Vec_new_isize(vSlice);
        
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vMem.close()
            return returnOpaque
        
        }
        fun newUsize(v: LongArray): Float64Vec {
            val (vMem, vSlice) = PrimitiveArrayTools.native(v)
            
            val returnVal = lib.Float64Vec_new_usize(vSlice);
        
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vMem.close()
            return returnOpaque
        
        }
        fun newF64BeBytes(v: ByteArray): Float64Vec {
            val (vMem, vSlice) = PrimitiveArrayTools.native(v)
            
            val returnVal = lib.Float64Vec_new_f64_be_bytes(vSlice);
        
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vMem.close()
            return returnOpaque
        
        }
        fun newFromOwned(v: DoubleArray): Float64Vec {
            val (vMem, vSlice) = PrimitiveArrayTools.native(v)
            
            val returnVal = lib.Float64Vec_new_from_owned(vSlice);
        
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            
            return returnOpaque
        
        }
    }
    fun asBoxedSlice(): DoubleArray {
        
        val returnVal = lib.Float64Vec_as_boxed_slice(handle);
        val string = PrimitiveArrayTools.getDoubleArray(returnVal)
        Native.free(Pointer.nativeValue(returnVal.data))
        return string
    }
    fun asSlice(): DoubleArray {
        
        val returnVal = lib.Float64Vec_as_slice(handle);
        return PrimitiveArrayTools.getDoubleArray(returnVal)
    }
    fun fillSlice(v: DoubleArray): Unit {
        val (vMem, vSlice) = PrimitiveArrayTools.native(v)
        
        val returnVal = lib.Float64Vec_fill_slice(handle, vSlice);
    }
    fun setValue(newSlice: DoubleArray): Unit {
        val (newSliceMem, newSliceSlice) = PrimitiveArrayTools.native(newSlice)
        
        val returnVal = lib.Float64Vec_set_value(handle, newSliceSlice);
    }
    fun toString_(): String {
        val writeable = DW.lib.diplomat_buffer_writeable_create(0)
        val returnVal = lib.Float64Vec_to_string(handle, writeable);
    
        val returnString = DW.writeableToString(writeable)
        DW.lib.diplomat_buffer_writeable_destroy(writeable)
        return returnString
    }
    fun borrow(): DoubleArray {
        
        val returnVal = lib.Float64Vec_borrow(handle);
        return PrimitiveArrayTools.getDoubleArray(returnVal)
    }
    fun get(i: Long): Double? {
        
        val returnVal = lib.Float64Vec_get(handle, i);
    
        if (returnVal == null) {
            return null
        } else {
                return returnVal
        }
    }

}
