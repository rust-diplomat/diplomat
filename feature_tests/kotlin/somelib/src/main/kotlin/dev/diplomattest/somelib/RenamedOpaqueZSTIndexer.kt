package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedOpaqueZSTIndexerLib: Library {
    fun namespace_OpaqueZSTIndexer_destroy(handle: Pointer)
    fun namespace_OpaqueZSTIndexer_new(): Pointer
    fun namespace_OpaqueZSTIndexer_index(handle: Pointer, idx: FFISizet): Pointer?
}

class RenamedOpaqueZSTIndexer internal constructor (
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

    private class RenamedOpaqueZSTIndexerCleaner(val handle: Pointer, val lib: RenamedOpaqueZSTIndexerLib) : Runnable {
        override fun run() {
            lib.namespace_OpaqueZSTIndexer_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, RenamedOpaqueZSTIndexer.RenamedOpaqueZSTIndexerCleaner(handle, RenamedOpaqueZSTIndexer.lib));
    }

    companion object {
        internal val libClass: Class<RenamedOpaqueZSTIndexerLib> = RenamedOpaqueZSTIndexerLib::class.java
        internal val lib: RenamedOpaqueZSTIndexerLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(): RenamedOpaqueZSTIndexer {
            
            val returnVal = lib.namespace_OpaqueZSTIndexer_new();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = RenamedOpaqueZSTIndexer(handle, selfEdges, true)
            return returnOpaque
        }
    }
    
    internal fun getInternal(idx: ULong): RenamedOpaqueZSTIndexer? {
        
        val returnVal = lib.namespace_OpaqueZSTIndexer_index(handle, FFISizet(idx));
        val selfEdges: List<Any> = listOf()
        val handle = returnVal ?: return null
        val returnOpaque = RenamedOpaqueZSTIndexer(handle, selfEdges, true)
        return returnOpaque
    }

    operator fun get(index: ULong): RenamedOpaqueZSTIndexer? {
        val returnVal = getInternal(index)
        if (returnVal == null) {
            throw IndexOutOfBoundsException("Index $index is out of bounds.")
        } else {
            return returnVal
        }
    }

}