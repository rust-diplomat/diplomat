package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedOpaqueRefIterableLib: Library {
    fun namespace_OpaqueRefIterable_destroy(handle: Pointer)
    fun namespace_OpaqueRefIterable_new(size: FFISizet): Pointer
    fun namespace_OpaqueRefIterable_iter(handle: Pointer): Pointer
}

class RenamedOpaqueRefIterable internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
): Iterable<RenamedOpaqueRefIteratorIteratorItem> {

    internal class RenamedOpaqueRefIterableCleaner(val handle: Pointer, val lib: RenamedOpaqueRefIterableLib) : Runnable {
        override fun run() {
            lib.namespace_OpaqueRefIterable_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<RenamedOpaqueRefIterableLib> = RenamedOpaqueRefIterableLib::class.java
        internal val lib: RenamedOpaqueRefIterableLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(size: ULong): RenamedOpaqueRefIterable {
            
            val returnVal = lib.namespace_OpaqueRefIterable_new(FFISizet(size));
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = RenamedOpaqueRefIterable(handle, selfEdges)
            CLEANER.register(returnOpaque, RenamedOpaqueRefIterable.RenamedOpaqueRefIterableCleaner(handle, RenamedOpaqueRefIterable.lib));
            return returnOpaque
        }
    }
    
    override fun iterator(): RenamedOpaqueRefIterator {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.namespace_OpaqueRefIterable_iter(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = RenamedOpaqueRefIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, RenamedOpaqueRefIterator.RenamedOpaqueRefIteratorCleaner(handle, RenamedOpaqueRefIterator.lib));
        return returnOpaque
    }

}