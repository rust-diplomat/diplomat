package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OptionStringLib: Library {
    fun OptionString_destroy(handle: Pointer)
    fun OptionString_new(diplomatStr: Slice): Pointer?
    fun OptionString_write(handle: Pointer, write: Pointer): ResultUnitUnit
    fun OptionString_borrow(handle: Pointer): OptionSlice
}

class OptionString internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class OptionStringCleaner(val handle: Pointer, val lib: OptionStringLib) : Runnable {
        override fun run() {
            lib.OptionString_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, OptionString.OptionStringCleaner(handle, OptionString.lib));
    }

    companion object {
        internal val libClass: Class<OptionStringLib> = OptionStringLib::class.java
        internal val lib: OptionStringLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(diplomatStr: String): OptionString? {
            val diplomatStrSliceMemory = PrimitiveArrayTools.borrowUtf8(diplomatStr)
            
            val returnVal = lib.OptionString_new(diplomatStrSliceMemory.slice);
            try {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal ?: return null
                val returnOpaque = OptionString(handle, selfEdges, true)
                return returnOpaque
            } finally {
                diplomatStrSliceMemory.close()
            }
        }
    }
    
    fun write(): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.OptionString_write(handle, write);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            
            val returnString = DW.writeToString(write)
            return returnString.ok()
        } else {
            return UnitError().err()
        }
    }
    
    fun borrow(): String? {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.OptionString_borrow(handle);
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }

}