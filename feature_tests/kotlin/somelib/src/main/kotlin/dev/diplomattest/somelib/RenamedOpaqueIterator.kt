package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedOpaqueIteratorLib: Library {
    fun namespace_OpaqueIterator_destroy(handle: Pointer)
    fun namespace_OpaqueIterator_next(handle: Pointer): Pointer?
}
typealias RenamedOpaqueIteratorIteratorItem = AttrOpaque1Renamed?

class RenamedOpaqueIterator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
): Iterator<AttrOpaque1Renamed?> {

    internal class RenamedOpaqueIteratorCleaner(val handle: Pointer, val lib: RenamedOpaqueIteratorLib) : Runnable {
        override fun run() {
            lib.namespace_OpaqueIterator_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<RenamedOpaqueIteratorLib> = RenamedOpaqueIteratorLib::class.java
        internal val lib: RenamedOpaqueIteratorLib = Native.load("diplomat_feature_tests", libClass)
    }
    
    internal fun nextInternal(): AttrOpaque1Renamed? {
        
        val returnVal = lib.namespace_OpaqueIterator_next(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal ?: return null
        val returnOpaque = AttrOpaque1Renamed(handle, selfEdges)
        CLEANER.register(returnOpaque, AttrOpaque1Renamed.AttrOpaque1RenamedCleaner(handle, AttrOpaque1Renamed.lib));
        return returnOpaque
    }

    var iterVal = nextInternal()

    override fun hasNext(): Boolean {
       return iterVal != null
    }

    override fun next(): AttrOpaque1Renamed?{
        val returnVal = iterVal
        if (returnVal == null) {
            throw NoSuchElementException()
        } else {
            iterVal = nextInternal()
            return returnVal
        }
    }

}