package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedTestOpaqueLib: Library {
    fun namespace_TestOpaque_destroy(handle: Pointer)
}

class RenamedTestOpaque internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class RenamedTestOpaqueCleaner(val handle: Pointer, val lib: RenamedTestOpaqueLib) : Runnable {
        override fun run() {
            lib.namespace_TestOpaque_destroy(handle)
        }
    }
    fun registerCleaner() {
        CLEANER.register(this, RenamedTestOpaque.RenamedTestOpaqueCleaner(handle, RenamedTestOpaque.lib));
    }

    companion object {
        internal val libClass: Class<RenamedTestOpaqueLib> = RenamedTestOpaqueLib::class.java
        internal val lib: RenamedTestOpaqueLib = Native.load("diplomat_feature_tests", libClass)
    }

}