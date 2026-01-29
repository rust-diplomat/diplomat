package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OpaqueThinIterLib: Library {
    fun OpaqueThinIter_destroy(handle: Pointer)
    fun OpaqueThinIter_next(handle: Pointer): Pointer?
}
typealias OpaqueThinIterIteratorItem = OpaqueThin?

class OpaqueThinIter internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
): Iterator<OpaqueThin?> {

    internal class OpaqueThinIterCleaner(val handle: Pointer, val lib: OpaqueThinIterLib) : Runnable {
        override fun run() {
            lib.OpaqueThinIter_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OpaqueThinIterLib> = OpaqueThinIterLib::class.java
        internal val lib: OpaqueThinIterLib = Native.load("diplomat_feature_tests", libClass)
    }
    
    internal fun nextInternal(): OpaqueThin? {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.OpaqueThinIter_next(handle);
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal ?: return null
        val returnOpaque = OpaqueThin(handle, selfEdges)
        return returnOpaque
    }

    var iterVal = nextInternal()

    override fun hasNext(): Boolean {
       return iterVal != null
    }

    override fun next(): OpaqueThin?{
        val returnVal = iterVal
        if (returnVal == null) {
            throw NoSuchElementException()
        } else {
            iterVal = nextInternal()
            return returnVal
        }
    }

}