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
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class TwoCleaner(val handle: Pointer, val lib: TwoLib) : Runnable {
        override fun run() {
            lib.Two_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, Two.TwoCleaner(handle, Two.lib));
    }

    companion object {
        internal val libClass: Class<TwoLib> = TwoLib::class.java
        internal val lib: TwoLib = Native.load("diplomat_feature_tests", libClass)
    }

}