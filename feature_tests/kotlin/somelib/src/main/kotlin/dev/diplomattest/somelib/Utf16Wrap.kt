package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface Utf16WrapLib: Library {
    fun Utf16Wrap_destroy(handle: Pointer)
    fun Utf16Wrap_from_utf16(input: Slice): Pointer
    fun Utf16Wrap_get_debug_str(handle: Pointer, write: Pointer): Unit
    fun Utf16Wrap_borrow_cont(handle: Pointer): Slice
}

class Utf16Wrap internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class Utf16WrapCleaner(val handle: Pointer, val lib: Utf16WrapLib) : Runnable {
        override fun run() {
            lib.Utf16Wrap_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<Utf16WrapLib> = Utf16WrapLib::class.java
        internal val lib: Utf16WrapLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun fromUtf16(input: String): Utf16Wrap {
            val inputSliceMemory = PrimitiveArrayTools.borrowUtf16(input)
            
            val returnVal = lib.Utf16Wrap_from_utf16(inputSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Utf16Wrap(handle, selfEdges)
            CLEANER.register(returnOpaque, Utf16Wrap.Utf16WrapCleaner(handle, Utf16Wrap.lib));
            inputSliceMemory?.close()
            return returnOpaque
        }
    }
    
    fun getDebugStr(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.Utf16Wrap_get_debug_str(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    fun borrowCont(): String {
        
        val returnVal = lib.Utf16Wrap_borrow_cont(handle);
            return PrimitiveArrayTools.getUtf16(returnVal)
    }

}