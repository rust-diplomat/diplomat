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
)  {

    internal class UnnamespacedCleaner(val handle: Pointer, val lib: UnnamespacedLib) : Runnable {
        override fun run() {
            lib.namespace_Unnamespaced_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<UnnamespacedLib> = UnnamespacedLib::class.java
        internal val lib: UnnamespacedLib = Native.load("somelib", libClass)
        @JvmStatic
        
        fun make(e: AttrEnum): Unnamespaced {
            
            val returnVal = lib.namespace_Unnamespaced_make(e.toNative());
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Unnamespaced(handle, selfEdges)
            CLEANER.register(returnOpaque, Unnamespaced.UnnamespacedCleaner(handle, Unnamespaced.lib));
            return returnOpaque
        }
    }
    
    fun useNamespaced(n: AttrOpaque1): Unit {
        
        val returnVal = lib.namespace_Unnamespaced_use_namespaced(handle, n.handle);
        
    }

}