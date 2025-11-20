package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DeprecatedOpaqueLib: Library {
    fun namespace_DeprecatedOpaque_destroy(handle: Pointer)
}

class DeprecatedOpaque internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class DeprecatedOpaqueCleaner(val handle: Pointer, val lib: DeprecatedOpaqueLib) : Runnable {
        override fun run() {
            lib.namespace_DeprecatedOpaque_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<DeprecatedOpaqueLib> = DeprecatedOpaqueLib::class.java
        internal val lib: DeprecatedOpaqueLib = Native.load("diplomat_feature_tests", libClass)
    }

}