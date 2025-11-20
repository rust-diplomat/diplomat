package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OpaqueThinVecLib: Library {
    fun OpaqueThinVec_destroy(handle: Pointer)
    fun OpaqueThinVec_create(a: Slice, b: Slice, c: Slice): Pointer
    fun OpaqueThinVec_iter(handle: Pointer): Pointer
    fun OpaqueThinVec_len(handle: Pointer): FFISizet
    fun OpaqueThinVec_get(handle: Pointer, idx: FFISizet): Pointer?
    fun OpaqueThinVec_first(handle: Pointer): Pointer?
}

class OpaqueThinVec internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
): Iterable<OpaqueThinIterIteratorItem> {

    internal class OpaqueThinVecCleaner(val handle: Pointer, val lib: OpaqueThinVecLib) : Runnable {
        override fun run() {
            lib.OpaqueThinVec_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OpaqueThinVecLib> = OpaqueThinVecLib::class.java
        internal val lib: OpaqueThinVecLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun create(a: IntArray, b: FloatArray, c: String): OpaqueThinVec {
            val (aMem, aSlice) = PrimitiveArrayTools.borrow(a)
            val (bMem, bSlice) = PrimitiveArrayTools.borrow(b)
            val (cMem, cSlice) = PrimitiveArrayTools.borrowUtf8(c)
            
            val returnVal = lib.OpaqueThinVec_create(aSlice, bSlice, cSlice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = OpaqueThinVec(handle, selfEdges)
            CLEANER.register(returnOpaque, OpaqueThinVec.OpaqueThinVecCleaner(handle, OpaqueThinVec.lib));
            if (aMem != null) aMem.close()
            if (bMem != null) bMem.close()
            if (cMem != null) cMem.close()
            return returnOpaque
        }
    }
    
    override fun iterator(): OpaqueThinIter {
        
        val returnVal = lib.OpaqueThinVec_iter(handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = OpaqueThinIter(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, OpaqueThinIter.OpaqueThinIterCleaner(handle, OpaqueThinIter.lib));
        return returnOpaque
    }
    
    fun len(): ULong {
        
        val returnVal = lib.OpaqueThinVec_len(handle);
        return (returnVal.toULong())
    }
    
    internal fun getInternal(idx: ULong): OpaqueThin? {
        
        val returnVal = lib.OpaqueThinVec_get(handle, FFISizet(idx));
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal ?: return null
        val returnOpaque = OpaqueThin(handle, selfEdges)
        return returnOpaque
    }
    
    fun first(): OpaqueThin? {
        
        val returnVal = lib.OpaqueThinVec_first(handle);
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal ?: return null
        val returnOpaque = OpaqueThin(handle, selfEdges)
        return returnOpaque
    }

    operator fun get(index: ULong): OpaqueThin? {
        val returnVal = getInternal(index)
        if (returnVal == null) {
            throw IndexOutOfBoundsException("Index $index is out of bounds.")
        } else {
            return returnVal
        }
    }

}