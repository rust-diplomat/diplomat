package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedMyIndexerLib: Library {
    fun namespace_MyIndexer_destroy(handle: Pointer)
    fun namespace_MyIndexer_new(v: Slice): Pointer
    fun namespace_MyIndexer_get(handle: Pointer, i: FFISizet): OptionSlice
}

class RenamedMyIndexer internal constructor (
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

    private class RenamedMyIndexerCleaner(val handle: Pointer, val lib: RenamedMyIndexerLib) : Runnable {
        override fun run() {
            lib.namespace_MyIndexer_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, RenamedMyIndexer.RenamedMyIndexerCleaner(handle, RenamedMyIndexer.lib));
    }

    companion object {
        internal val libClass: Class<RenamedMyIndexerLib> = RenamedMyIndexerLib::class.java
        internal val lib: RenamedMyIndexerLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(v: Array<String>): RenamedMyIndexer {
            val vSliceMemory = PrimitiveArrayTools.borrowUtf8s(v)
            
            val returnVal = lib.namespace_MyIndexer_new(vSliceMemory.slice);
            try {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal 
                val returnOpaque = RenamedMyIndexer(handle, selfEdges, true)
                return returnOpaque
            } finally {
                vSliceMemory.close()
            }
        }
    }
    
    internal fun getInternal(i: ULong): String? {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.namespace_MyIndexer_get(handle, FFISizet(i));
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }

    operator fun get(index: ULong): String {
        val returnVal = getInternal(index)
        if (returnVal == null) {
            throw IndexOutOfBoundsException("Index $index is out of bounds.")
        } else {
            return returnVal
        }
    }

}