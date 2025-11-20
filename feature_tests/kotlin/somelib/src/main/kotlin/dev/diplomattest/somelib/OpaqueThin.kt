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
    fun OpaqueThin_c(handle: Pointer, write: Pointer): Unit
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
        internal val lib: OpaqueThinLib = Native.load("diplomat_feature_tests", libClass)
    }
    
    fun a(): Int {
        
        val returnVal = lib.OpaqueThin_a(handle);
        return (returnVal)
    }
    
    fun b(): Float {
        
        val returnVal = lib.OpaqueThin_b(handle);
        return (returnVal)
    }
    
    fun c(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.OpaqueThin_c(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}