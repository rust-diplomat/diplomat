package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedDeprecatedOpaqueLib: Library {
    fun namespace_DeprecatedOpaque_destroy(handle: Pointer)
}

class RenamedDeprecatedOpaque internal constructor (
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

    private class RenamedDeprecatedOpaqueCleaner(val handle: Pointer, val lib: RenamedDeprecatedOpaqueLib) : Runnable {
        override fun run() {
            lib.namespace_DeprecatedOpaque_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, RenamedDeprecatedOpaque.RenamedDeprecatedOpaqueCleaner(handle, RenamedDeprecatedOpaque.lib));
    }

    companion object {
        internal val libClass: Class<RenamedDeprecatedOpaqueLib> = RenamedDeprecatedOpaqueLib::class.java
        internal val lib: RenamedDeprecatedOpaqueLib = Native.load("diplomat_feature_tests", libClass)
    }

}