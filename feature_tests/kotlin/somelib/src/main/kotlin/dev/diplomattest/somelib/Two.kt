package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TwoLib: Library {
    fun Two_destroy(handle: Pointer)
}

class Two internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
    internal val bEdges: List<Any?>,
)  {

    internal class TwoCleaner(val handle: Pointer, val lib: TwoLib) : Runnable {
        override fun run() {
            lib.Two_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<TwoLib> = TwoLib::class.java
        internal val lib: TwoLib = Native.load("somelib", libClass)
    }

}