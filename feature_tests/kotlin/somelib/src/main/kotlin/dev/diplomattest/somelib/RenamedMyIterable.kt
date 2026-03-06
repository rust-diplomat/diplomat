package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedMyIterableLib: Library {
    fun namespace_MyIterable_destroy(handle: Pointer)
    fun namespace_MyIterable_new(x: Slice): Pointer
    fun namespace_MyIterable_iter(handle: Pointer): Pointer
}

class RenamedMyIterable internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal var owned: Boolean,
): Iterable<RenamedMyIteratorIteratorItem> {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class RenamedMyIterableCleaner(val handle: Pointer, val lib: RenamedMyIterableLib) : Runnable {
        override fun run() {
            lib.namespace_MyIterable_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, RenamedMyIterable.RenamedMyIterableCleaner(handle, RenamedMyIterable.lib));
    }

    companion object {
        internal val libClass: Class<RenamedMyIterableLib> = RenamedMyIterableLib::class.java
        internal val lib: RenamedMyIterableLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(x: UByteArray): RenamedMyIterable {
            val xSliceMemory = PrimitiveArrayTools.borrow(x)
            
            val returnVal = lib.namespace_MyIterable_new(xSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = RenamedMyIterable(handle, selfEdges, true)
            xSliceMemory.close()
            return returnOpaque
        }
    }
    
    override fun iterator(): RenamedMyIterator {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.namespace_MyIterable_iter(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = RenamedMyIterator(handle, selfEdges, aEdges, true)
        return returnOpaque
    }

}