package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OpaqueRefIteratorLib: Library {
    fun namespace_OpaqueRefIterator_destroy(handle: Pointer)
    fun namespace_OpaqueRefIterator_next(handle: Pointer): Pointer?
}
typealias OpaqueRefIteratorIteratorItem = AttrOpaque1?

class OpaqueRefIterator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
): Iterator<AttrOpaque1?> {

    internal class OpaqueRefIteratorCleaner(val handle: Pointer, val lib: OpaqueRefIteratorLib) : Runnable {
        override fun run() {
            lib.namespace_OpaqueRefIterator_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OpaqueRefIteratorLib> = OpaqueRefIteratorLib::class.java
        internal val lib: OpaqueRefIteratorLib = Native.load("somelib", libClass)
    }
    
    internal fun nextInternal(): AttrOpaque1? {
        
        val returnVal = lib.namespace_OpaqueRefIterator_next(handle);
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal ?: return null
        val returnOpaque = AttrOpaque1(handle, selfEdges)
        return returnOpaque
    }

    var iterVal = nextInternal()

    override fun hasNext(): Boolean {
       return iterVal != null
    }

    override fun next(): AttrOpaque1?{
        val returnVal = iterVal
        if (returnVal == null) {
            throw NoSuchElementException()
        } else {
            iterVal = nextInternal()
            return returnVal
        }
    }

}