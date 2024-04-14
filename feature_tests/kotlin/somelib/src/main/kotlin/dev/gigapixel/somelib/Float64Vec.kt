package dev.gigapixel.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


interface Float64VecLib: Library {
    fun Float64Vec_destroy(handle: Long)
    fun Float64Vec_new(v: Slice): Long
    fun Float64Vec_new_bool(v: Slice): Long
    fun Float64Vec_new_i16(v: Slice): Long
    fun Float64Vec_new_u16(v: Slice): Long
    fun Float64Vec_new_isize(v: Slice): Long
    fun Float64Vec_new_usize(v: Slice): Long
    fun Float64Vec_new_f64_be_bytes(v: Slice): Long
    fun Float64Vec_fill_slice(handle: Long, v: Slice): Unit
    fun Float64Vec_set_value(handle: Long, newSlice: Slice): Unit
    fun Float64Vec_to_string(handle: Long, writeable: Pointer): Unit
}

class Float64Vec internal constructor (
    internal val handle: Long,
    internal val selfEdges: List<Any>) {

    internal class Float64VecCleaner(val handle: Long, val lib: Float64VecLib) : Runnable {
        override fun run() {
            lib.Float64Vec_destroy(handle)
        }
    }

    companion object {
        val libClass: Class<Float64VecLib> = Float64VecLib::class.java
        val lib: Float64VecLib = Native.load("somelib", libClass)
        fun new_(v: DoubleArray): Float64Vec {
        
            val (vMem, vSlice) = PrimitiveArrayTools.native(v)
            
            val returnVal = lib.Float64Vec_new(vSlice);
        
            val selfEdges: List<Any> = listOf()
            val handle = returnVal
            val returnOpaque = Float64Vec(handle, selfEdges)
            CLEANER.register(returnOpaque, Float64Vec.Float64VecCleaner(handle, Float64Vec.lib));
            vMem.close()
            return returnOpaque
        
        }
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

}
