package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface Float64VecLib: Library {
    fun Float64Vec_destroy(handle: Pointer)
    fun Float64Vec_new_bool(v: Slice): Pointer
    fun Float64Vec_new_i16(v: Slice): Pointer
    fun Float64Vec_new_u16(v: Slice): Pointer
    fun Float64Vec_new_isize(v: Slice): Pointer
    fun Float64Vec_new_usize(v: Slice): Pointer
    fun Float64Vec_new_f64_be_bytes(v: Slice): Pointer
    fun Float64Vec_new_from_owned(v: Slice): Pointer
    fun Float64Vec_as_slice(handle: Pointer): Slice
    fun Float64Vec_fill_slice(handle: Pointer, v: Slice): Unit
    fun Float64Vec_set_value(handle: Pointer, newSlice: Slice): Unit
    fun Float64Vec_to_string(handle: Pointer, write: Pointer): Unit
    fun Float64Vec_borrow(handle: Pointer): Slice
    fun Float64Vec_get(handle: Pointer, i: FFISizet): OptionDouble
}

class Float64Vec internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class Float64VecCleaner(val handle: Pointer, val lib: Float64VecLib) : Runnable {
        override fun run() {
            lib.Float64Vec_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<Float64VecLib> = Float64VecLib::class.java
        internal val lib: Float64VecLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun newBool(v: BooleanArray): Float64Vec {
            val vSliceMemory = PrimitiveArrayTools.borrow(v)
            
            val returnVal = lib.Float64Vec_new_bool(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vSliceMemory?.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun newI16(v: ShortArray): Float64Vec {
            val vSliceMemory = PrimitiveArrayTools.borrow(v)
            
            val returnVal = lib.Float64Vec_new_i16(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vSliceMemory?.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun newU16(v: UShortArray): Float64Vec {
            val vSliceMemory = PrimitiveArrayTools.borrow(v)
            
            val returnVal = lib.Float64Vec_new_u16(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vSliceMemory?.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun newIsize(v: LongArray): Float64Vec {
            val vSliceMemory = PrimitiveArrayTools.borrow(v)
            
            val returnVal = lib.Float64Vec_new_isize(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vSliceMemory?.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun newUsize(v: ULongArray): Float64Vec {
            val vSliceMemory = PrimitiveArrayTools.borrow(v)
            
            val returnVal = lib.Float64Vec_new_usize(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vSliceMemory?.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun newF64BeBytes(v: ByteArray): Float64Vec {
            val vSliceMemory = PrimitiveArrayTools.borrow(v)
            
            val returnVal = lib.Float64Vec_new_f64_be_bytes(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vSliceMemory?.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun newFromOwned(v: DoubleArray): Float64Vec {
            val vSliceMemory = PrimitiveArrayTools.move(v)
            
            val returnVal = lib.Float64Vec_new_from_owned(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            return returnOpaque
        }
    }
    
    fun asSlice(): DoubleArray {
        
        val returnVal = lib.Float64Vec_as_slice(handle);
            return PrimitiveArrayTools.getDoubleArray(returnVal)
    }
    
    fun fillSlice(v: DoubleArray): Unit {
        val vSliceMemory = PrimitiveArrayTools.borrow(v)
        
        val returnVal = lib.Float64Vec_fill_slice(handle, vSliceMemory.slice);
        
    }
    
    fun setValue(newSlice: DoubleArray): Unit {
        val newSliceSliceMemory = PrimitiveArrayTools.borrow(newSlice)
        
        val returnVal = lib.Float64Vec_set_value(handle, newSliceSliceMemory.slice);
        
    }
    
    override fun toString(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.Float64Vec_to_string(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    fun borrow(): DoubleArray {
        
        val returnVal = lib.Float64Vec_borrow(handle);
            return PrimitiveArrayTools.getDoubleArray(returnVal)
    }
    
    internal fun getInternal(i: ULong): Double? {
        
        val returnVal = lib.Float64Vec_get(handle, FFISizet(i));
        return returnVal.option()
    }

    operator fun get(index: ULong): Double {
        val returnVal = getInternal(index)
        if (returnVal == null) {
            throw IndexOutOfBoundsException("Index $index is out of bounds.")
        } else {
            return returnVal
        }
    }

}