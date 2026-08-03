package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MutableBytesLib: Library {
    fun MutableBytes_destroy(handle: Pointer)
    fun MutableBytes_get_as_bytes(handle: Pointer): Slice
    fun MutableBytes_set_bytes(handle: Pointer, newBytes: Slice): Unit
    fun MutableBytes_new(): Pointer
}

class MutableBytes internal constructor (
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

    private class MutableBytesCleaner(val handle: Pointer, val lib: MutableBytesLib) : Runnable {
        override fun run() {
            lib.MutableBytes_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, MutableBytes.MutableBytesCleaner(handle, MutableBytes.lib));
    }

    companion object {
        internal val libClass: Class<MutableBytesLib> = MutableBytesLib::class.java
        internal val lib: MutableBytesLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(): MutableBytes {
            
            val returnVal = lib.MutableBytes_new();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MutableBytes(handle, selfEdges, true)
            return returnOpaque
        }
    }
    
    fun getAsBytes(): UByteArray {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.MutableBytes_get_as_bytes(handle);
            return PrimitiveArrayTools.getUByteArray(returnVal)
    }
    
    fun setBytes(newBytes: UByteArray): Unit {
        val newBytesSliceMemory = PrimitiveArrayTools.borrow(newBytes)
        
        val returnVal = lib.MutableBytes_set_bytes(handle, newBytesSliceMemory.slice);
        try {
            
        } finally {
            newBytesSliceMemory.close()
        }
    }

}