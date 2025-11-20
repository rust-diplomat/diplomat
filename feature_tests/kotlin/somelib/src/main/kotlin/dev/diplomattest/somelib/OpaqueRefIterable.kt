package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OpaqueRefIterableLib: Library {
    fun namespace_OpaqueRefIterable_destroy(handle: Pointer)
    fun namespace_OpaqueRefIterable_new(size: FFISizet): Pointer
    fun namespace_OpaqueRefIterable_iter(handle: Pointer): Pointer
}

class OpaqueRefIterable internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
): Iterable<OpaqueRefIteratorIteratorItem> {

    internal class OpaqueRefIterableCleaner(val handle: Pointer, val lib: OpaqueRefIterableLib) : Runnable {
        override fun run() {
            lib.namespace_OpaqueRefIterable_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OpaqueRefIterableLib> = OpaqueRefIterableLib::class.java
        internal val lib: OpaqueRefIterableLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(size: ULong): OpaqueRefIterable {
            
            val returnVal = lib.namespace_OpaqueRefIterable_new(FFISizet(size));
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = OpaqueRefIterable(handle, selfEdges)
            CLEANER.register(returnOpaque, OpaqueRefIterable.OpaqueRefIterableCleaner(handle, OpaqueRefIterable.lib));
            return returnOpaque
        }
    }
    
    override fun iterator(): OpaqueRefIterator {
        
        val returnVal = lib.namespace_OpaqueRefIterable_iter(handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = OpaqueRefIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, OpaqueRefIterator.OpaqueRefIteratorCleaner(handle, OpaqueRefIterator.lib));
        return returnOpaque
    }

}