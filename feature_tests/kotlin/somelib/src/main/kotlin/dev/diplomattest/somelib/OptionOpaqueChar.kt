package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OptionOpaqueCharLib: Library {
    fun OptionOpaqueChar_destroy(handle: Pointer)
    fun OptionOpaqueChar_assert_char(handle: Pointer, ch: Int): Unit
}

class OptionOpaqueChar internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class OptionOpaqueCharCleaner(val handle: Pointer, val lib: OptionOpaqueCharLib) : Runnable {
        override fun run() {
            lib.OptionOpaqueChar_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OptionOpaqueCharLib> = OptionOpaqueCharLib::class.java
        internal val lib: OptionOpaqueCharLib = Native.load("diplomat_feature_tests", libClass)
    }
    
    fun assertChar(ch: Int): Unit {
        
        val returnVal = lib.OptionOpaqueChar_assert_char(handle, ch);
        
    }

}