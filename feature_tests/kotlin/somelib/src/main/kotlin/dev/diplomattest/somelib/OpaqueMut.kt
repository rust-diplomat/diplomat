package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OpaqueMutLib: Library {
    fun OpaqueMut_destroy(handle: Pointer)
}

class OpaqueMut internal constructor (
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

    private class OpaqueMutCleaner(val handle: Pointer, val lib: OpaqueMutLib) : Runnable {
        override fun run() {
            lib.OpaqueMut_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, OpaqueMut.OpaqueMutCleaner(handle, OpaqueMut.lib));
    }

    companion object {
        internal val libClass: Class<OpaqueMutLib> = OpaqueMutLib::class.java
        internal val lib: OpaqueMutLib = Native.load("diplomat_feature_tests", libClass)
    }

}