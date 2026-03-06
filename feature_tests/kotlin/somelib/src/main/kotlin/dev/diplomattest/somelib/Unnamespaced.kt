package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface UnnamespacedLib: Library {
    fun namespace_Unnamespaced_destroy(handle: Pointer)
    fun namespace_Unnamespaced_make(e: Int): Pointer
    fun namespace_Unnamespaced_use_namespaced(handle: Pointer, n: Pointer): Unit
}

class Unnamespaced internal constructor (
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

    private class UnnamespacedCleaner(val handle: Pointer, val lib: UnnamespacedLib) : Runnable {
        override fun run() {
            lib.namespace_Unnamespaced_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, Unnamespaced.UnnamespacedCleaner(handle, Unnamespaced.lib));
    }

    companion object {
        internal val libClass: Class<UnnamespacedLib> = UnnamespacedLib::class.java
        internal val lib: UnnamespacedLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun make(e: RenamedAttrEnum): Unnamespaced {
            
            val returnVal = lib.namespace_Unnamespaced_make(e.toNative());
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Unnamespaced(handle, selfEdges, true)
            return returnOpaque
        }
    }
    
    fun useNamespaced(n: AttrOpaque1Renamed): Unit {
        
        val returnVal = lib.namespace_Unnamespaced_use_namespaced(handle, n.handle);
        
    }

}