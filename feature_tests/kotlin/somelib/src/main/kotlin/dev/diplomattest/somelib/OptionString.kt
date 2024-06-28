package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


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
    internal val selfEdges: List<Any>
)  {

    internal class OptionStringCleaner(val handle: Pointer, val lib: OptionStringLib) : Runnable {
        override fun run() {
            lib.OptionString_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OptionStringLib> = OptionStringLib::class.java
        internal val lib: OptionStringLib = Native.load("somelib", libClass)
        
        fun new_(diplomatStr: String): OptionString? {
            val (diplomatStrMem, diplomatStrSlice) = PrimitiveArrayTools.readUtf8(diplomatStr)
            
            val returnVal = lib.OptionString_new(diplomatStrSlice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal ?: return null
            val returnOpaque = OptionString(handle, selfEdges)
            CLEANER.register(returnOpaque, OptionString.OptionStringCleaner(handle, OptionString.lib));
            diplomatStrMem.close()
            return returnOpaque
        }
    }
    
    fun write(): Res<String, Unit> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.OptionString_write(handle, write);
        if (returnVal.isOk == 1.toByte()) {
            
            val returnString = DW.writeToString(write)
            return returnString.ok()
        } else {
            return Err(Unit)
        }
    }
    
    fun borrow(): String? {
        
        val returnVal = lib.OptionString_borrow(handle);
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }

}
