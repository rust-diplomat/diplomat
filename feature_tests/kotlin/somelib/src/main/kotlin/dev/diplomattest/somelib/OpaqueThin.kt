package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OpaqueThinLib: Library {
    fun OpaqueThin_destroy(handle: Pointer)
    fun OpaqueThin_a(handle: Pointer): Int
    fun OpaqueThin_b(handle: Pointer): Float
}

class OpaqueThin internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class OpaqueThinCleaner(val handle: Pointer, val lib: OpaqueThinLib) : Runnable {
        override fun run() {
            lib.OpaqueThin_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OpaqueThinLib> = OpaqueThinLib::class.java
        internal val lib: OpaqueThinLib = Native.load("somelib", libClass)
    }
    
    fun a(): Int {
        
        val returnVal = lib.OpaqueThin_a(handle);
        return (returnVal)
    }
    
    fun b(): Float {
        
        val returnVal = lib.OpaqueThin_b(handle);
        return (returnVal)
    }

}