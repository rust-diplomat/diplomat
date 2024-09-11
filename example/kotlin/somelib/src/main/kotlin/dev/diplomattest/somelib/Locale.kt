package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure


internal interface LocaleLib: Library {
    fun icu4x_Locale_destroy_mv1(handle: Pointer)
    fun icu4x_Locale_new_mv1(name: Slice): Pointer
}

class Locale internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
)  {
    
    internal class LocaleCleaner(val handle: Pointer, val lib: LocaleLib) : Runnable {
        override fun run() {
            lib.icu4x_Locale_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<LocaleLib> = LocaleLib::class.java
        internal val lib: LocaleLib = Native.load("somelib", libClass)
        
        fun new_(name: String): Locale {
            val (nameMem, nameSlice) = PrimitiveArrayTools.readUtf8(name)
            
            val returnVal = lib.icu4x_Locale_new_mv1(nameSlice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Locale(handle, selfEdges)
            CLEANER.register(returnOpaque, Locale.LocaleCleaner(handle, Locale.lib));
            nameMem.close()
            return returnOpaque
        }
    }

}
