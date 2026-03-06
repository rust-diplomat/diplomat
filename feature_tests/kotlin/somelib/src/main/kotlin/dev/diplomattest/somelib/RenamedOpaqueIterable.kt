package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedOpaqueIterableLib: Library {
    fun namespace_OpaqueIterable_destroy(handle: Pointer)
    fun namespace_OpaqueIterable_new(size: FFISizet): Pointer
    fun namespace_OpaqueIterable_iter(handle: Pointer): Pointer
}

class RenamedOpaqueIterable internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
): Iterable<RenamedOpaqueIteratorIteratorItem> {

    internal class RenamedOpaqueIterableCleaner(val handle: Pointer, val lib: RenamedOpaqueIterableLib) : Runnable {
        override fun run() {
            lib.namespace_OpaqueIterable_destroy(handle)
        }
    }
    fun registerCleaner() {
        CLEANER.register(this, RenamedOpaqueIterable.RenamedOpaqueIterableCleaner(handle, RenamedOpaqueIterable.lib));
    }

    companion object {
        internal val libClass: Class<RenamedOpaqueIterableLib> = RenamedOpaqueIterableLib::class.java
        internal val lib: RenamedOpaqueIterableLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(size: ULong): RenamedOpaqueIterable {
            
            val returnVal = lib.namespace_OpaqueIterable_new(FFISizet(size));
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = RenamedOpaqueIterable(handle, selfEdges)
            returnOpaque.registerCleaner()
            return returnOpaque
        }
    }
    
    override fun iterator(): RenamedOpaqueIterator {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.namespace_OpaqueIterable_iter(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = RenamedOpaqueIterator(handle, selfEdges, aEdges)
        returnOpaque.registerCleaner()
        return returnOpaque
    }

}