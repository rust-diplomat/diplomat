package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedAttrOpaque2Lib: Library {
    fun namespace_AttrOpaque2_destroy(handle: Pointer)
}

class RenamedAttrOpaque2 internal constructor (
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

    private class RenamedAttrOpaque2Cleaner(val handle: Pointer, val lib: RenamedAttrOpaque2Lib) : Runnable {
        override fun run() {
            lib.namespace_AttrOpaque2_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, RenamedAttrOpaque2.RenamedAttrOpaque2Cleaner(handle, RenamedAttrOpaque2.lib));
    }

    companion object {
        internal val libClass: Class<RenamedAttrOpaque2Lib> = RenamedAttrOpaque2Lib::class.java
        internal val lib: RenamedAttrOpaque2Lib = Native.load("diplomat_feature_tests", libClass)
    }

}