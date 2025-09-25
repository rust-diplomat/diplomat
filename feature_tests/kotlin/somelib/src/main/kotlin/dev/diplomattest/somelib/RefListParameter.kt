package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RefListParameterLib: Library {
    fun RefListParameter_destroy(handle: Pointer)
}

class RefListParameter internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class RefListParameterCleaner(val handle: Pointer, val lib: RefListParameterLib) : Runnable {
        override fun run() {
            lib.RefListParameter_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<RefListParameterLib> = RefListParameterLib::class.java
        internal val lib: RefListParameterLib = Native.load("somelib", libClass)
    }

}