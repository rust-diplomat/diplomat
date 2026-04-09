package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedMixinTestLib: Library {
    fun namespace_MixinTest_destroy(handle: Pointer)
    fun namespace_MixinTest_hello(write: Pointer): Unit
}

class RenamedMixinTest internal constructor (
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

    private class RenamedMixinTestCleaner(val handle: Pointer, val lib: RenamedMixinTestLib) : Runnable {
        override fun run() {
            lib.namespace_MixinTest_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, RenamedMixinTest.RenamedMixinTestCleaner(handle, RenamedMixinTest.lib));
    }

    companion object {
        internal val libClass: Class<RenamedMixinTestLib> = RenamedMixinTestLib::class.java
        internal val lib: RenamedMixinTestLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun hello(): String {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.namespace_MixinTest_hello(write);
            
            val returnString = DW.writeToString(write)
            return returnString
        }
    }

}