package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedOpaqueRefIteratorLib: Library {
    fun namespace_OpaqueRefIterator_destroy(handle: Pointer)
    fun namespace_OpaqueRefIterator_next(handle: Pointer): Pointer?
}
typealias RenamedOpaqueRefIteratorIteratorItem = AttrOpaque1Renamed?

class RenamedOpaqueRefIterator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
    internal var owned: Boolean,
): Iterator<AttrOpaque1Renamed?> {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class RenamedOpaqueRefIteratorCleaner(val handle: Pointer, val lib: RenamedOpaqueRefIteratorLib) : Runnable {
        override fun run() {
            lib.namespace_OpaqueRefIterator_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, RenamedOpaqueRefIterator.RenamedOpaqueRefIteratorCleaner(handle, RenamedOpaqueRefIterator.lib));
    }

    companion object {
        internal val libClass: Class<RenamedOpaqueRefIteratorLib> = RenamedOpaqueRefIteratorLib::class.java
        internal val lib: RenamedOpaqueRefIteratorLib = Native.load("diplomat_feature_tests", libClass)
    }
    
    internal fun nextInternal(): AttrOpaque1Renamed? {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.namespace_OpaqueRefIterator_next(handle);
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal ?: return null
        val returnOpaque = AttrOpaque1Renamed(handle, selfEdges, false)
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