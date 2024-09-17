package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure


internal interface FixedDecimalFormatterLib: Library {
    fun icu4x_FixedDecimalFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_FixedDecimalFormatter_try_new_mv1(locale: Pointer, provider: Pointer, options: FixedDecimalFormatterOptionsNative): ResultPointerUnit
    fun icu4x_FixedDecimalFormatter_format_write_mv1(handle: Pointer, value: Pointer, write: Pointer): Unit
}

class FixedDecimalFormatter internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
)  {

    internal class FixedDecimalFormatterCleaner(val handle: Pointer, val lib: FixedDecimalFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_FixedDecimalFormatter_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<FixedDecimalFormatterLib> = FixedDecimalFormatterLib::class.java
        internal val lib: FixedDecimalFormatterLib = Native.load("somelib", libClass)
        
        fun tryNew(locale: Locale, provider: DataProvider, options: FixedDecimalFormatterOptions): Res<FixedDecimalFormatter, Unit> {
            
            val returnVal = lib.icu4x_FixedDecimalFormatter_try_new_mv1(locale.handle, provider.handle, options.nativeStruct);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = FixedDecimalFormatter(handle, selfEdges)
                CLEANER.register(returnOpaque, FixedDecimalFormatter.FixedDecimalFormatterCleaner(handle, FixedDecimalFormatter.lib));
                return returnOpaque.ok()
            } else {
                return Err(Unit)
            }
        }
    }
    
    fun formatWrite(value: FixedDecimal): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_FixedDecimalFormatter_format_write_mv1(handle, value.handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}
