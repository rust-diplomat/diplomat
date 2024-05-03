package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface ICU4XFixedDecimalFormatterLib: Library {
    fun ICU4XFixedDecimalFormatter_destroy(handle: Pointer)
    fun ICU4XFixedDecimalFormatter_try_new(locale: Pointer, provider: Pointer, options: ICU4XFixedDecimalFormatterOptionsNative): ResultPointerUnit
    fun ICU4XFixedDecimalFormatter_format_write(handle: Pointer, value: Pointer, writeable: Pointer): Unit
}

class ICU4XFixedDecimalFormatter internal constructor (
    internal val handle: Pointer,

    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>) {

    internal class ICU4XFixedDecimalFormatterCleaner(val handle: Pointer, val lib: ICU4XFixedDecimalFormatterLib) : Runnable {
        override fun run() {
            lib.ICU4XFixedDecimalFormatter_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<ICU4XFixedDecimalFormatterLib> = ICU4XFixedDecimalFormatterLib::class.java
        internal val lib: ICU4XFixedDecimalFormatterLib = Native.load("somelib", libClass)
        fun tryNew(locale: ICU4XLocale, provider: ICU4XDataProvider, options: ICU4XFixedDecimalFormatterOptions): Res<ICU4XFixedDecimalFormatter, Unit> {
            
            val returnVal = lib.ICU4XFixedDecimalFormatter_try_new(locale.handle, provider.handle, options.nativeStruct);
        
            if (returnVal.isOk == 1.toByte()) {
                
            val selfEdges: List<Any> = listOf()
            val handle = returnVal.union.ok 
            val returnOpaque = ICU4XFixedDecimalFormatter(handle, selfEdges)
            CLEANER.register(returnOpaque, ICU4XFixedDecimalFormatter.ICU4XFixedDecimalFormatterCleaner(handle, ICU4XFixedDecimalFormatter.lib));
            
            return returnOpaque.ok()
            
            } else {
                return Err(Unit)
            }
                            
        }
    }
    fun formatWrite(value: ICU4XFixedDecimal): String {
        val writeable = DW.lib.diplomat_buffer_writeable_create(0)
        val returnVal = lib.ICU4XFixedDecimalFormatter_format_write(handle, value.handle, writeable);
    
        val returnString = DW.writeableToString(writeable)
        DW.lib.diplomat_buffer_writeable_destroy(writeable)
        return returnString
    }

}
