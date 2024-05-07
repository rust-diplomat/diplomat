package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface OpaqueIterableLib: Library {
    fun OpaqueIterable_destroy(handle: Pointer)
    fun namespace_OpaqueIterable_iter(handle: Pointer): Pointer
}

class OpaqueIterable internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
): Iterable<OpaqueIteratorIteratorItem> {

    internal class OpaqueIterableCleaner(val handle: Pointer, val lib: OpaqueIterableLib) : Runnable {
        override fun run() {
            lib.OpaqueIterable_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OpaqueIterableLib> = OpaqueIterableLib::class.java
        internal val lib: OpaqueIterableLib = Native.load("somelib", libClass)
    }
    
    override fun iterator(): OpaqueIterator {
        val returnVal = lib.namespace_OpaqueIterable_iter(handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any> = listOf(this)
        val handle = returnVal 
        val returnOpaque = OpaqueIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, OpaqueIterator.OpaqueIteratorCleaner(handle, OpaqueIterator.lib));
        
        return returnOpaque
    }

}
