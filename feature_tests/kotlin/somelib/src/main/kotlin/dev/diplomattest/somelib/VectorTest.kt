package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface VectorTestLib: Library {
    fun namespace_VectorTest_destroy(handle: Pointer)
    fun namespace_VectorTest_new(): Pointer
    fun namespace_VectorTest_len(handle: Pointer): FFISizet
    fun namespace_VectorTest_get(handle: Pointer, idx: FFISizet): OptionDouble
    fun namespace_VectorTest_push(handle: Pointer, value: Double): Unit
}

class VectorTest internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class VectorTestCleaner(val handle: Pointer, val lib: VectorTestLib) : Runnable {
        override fun run() {
            lib.namespace_VectorTest_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<VectorTestLib> = VectorTestLib::class.java
        internal val lib: VectorTestLib = Native.load("somelib", libClass)
        @JvmStatic
        
        fun new_(): VectorTest {
            
            val returnVal = lib.namespace_VectorTest_new();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = VectorTest(handle, selfEdges)
            CLEANER.register(returnOpaque, VectorTest.VectorTestCleaner(handle, VectorTest.lib));
            return returnOpaque
        }
    }
    
    fun len(): ULong {
        
        val returnVal = lib.namespace_VectorTest_len(handle);
        return (returnVal.toULong())
    }
    
    internal fun getInternal(idx: ULong): Double? {
        
        val returnVal = lib.namespace_VectorTest_get(handle, FFISizet(idx));
        return returnVal.option()
    }
    
    fun push(value: Double): Unit {
        
        val returnVal = lib.namespace_VectorTest_push(handle, value);
        
    }

    operator fun get(index: ULong): Double {
        val returnVal = getInternal(index)
        if (returnVal == null) {
            throw IndexOutOfBoundsException("Index $index is out of bounds.")
        } else {
            return returnVal
        }
    }

}