package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface ICU4XLocaleLib: Library {
    fun ICU4XLocale_destroy(handle: Pointer)
    fun ICU4XLocale_new(name: Slice): Pointer
}

class ICU4XLocale internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
)  {

    internal class ICU4XLocaleCleaner(val handle: Pointer, val lib: ICU4XLocaleLib) : Runnable {
        override fun run() {
            lib.ICU4XLocale_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<ICU4XLocaleLib> = ICU4XLocaleLib::class.java
        internal val lib: ICU4XLocaleLib = Native.load("somelib", libClass)
        
        fun new_(name: String): ICU4XLocale {
            val (nameMem, nameSlice) = PrimitiveArrayTools.readUtf8(name)
            val returnVal = lib.ICU4XLocale_new(nameSlice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = ICU4XLocale(handle, selfEdges)
            CLEANER.register(returnOpaque, ICU4XLocale.ICU4XLocaleCleaner(handle, ICU4XLocale.lib));
            nameMem.close()
            return returnOpaque
        }
    }

}
