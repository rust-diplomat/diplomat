package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MethodOverloadingLib: Library {
    fun MethodOverloading_destroy(handle: Pointer)
    fun MethodOverloading_from_int32(v: Int): Pointer
    fun MethodOverloading_from_int64(v: Long): Pointer
    fun MethodOverloading_from_uint32(v: FFIUint32): Pointer
}

class MethodOverloading internal constructor (
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

    private class MethodOverloadingCleaner(val handle: Pointer, val lib: MethodOverloadingLib) : Runnable {
        override fun run() {
            lib.MethodOverloading_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, MethodOverloading.MethodOverloadingCleaner(handle, MethodOverloading.lib));
    }

    companion object {
        internal val libClass: Class<MethodOverloadingLib> = MethodOverloadingLib::class.java
        internal val lib: MethodOverloadingLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun from(v: Int): MethodOverloading {
            
            val returnVal = lib.MethodOverloading_from_int32(v);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MethodOverloading(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        fun from(v: Long): MethodOverloading {
            
            val returnVal = lib.MethodOverloading_from_int64(v);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MethodOverloading(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        fun from(v: UInt): MethodOverloading {
            
            val returnVal = lib.MethodOverloading_from_uint32(FFIUint32(v));
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MethodOverloading(handle, selfEdges, true)
            return returnOpaque
        }
    }

}