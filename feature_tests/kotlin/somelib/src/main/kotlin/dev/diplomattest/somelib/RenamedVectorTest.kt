package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedVectorTestLib: Library {
    fun namespace_VectorTest_destroy(handle: Pointer)
    fun namespace_VectorTest_new(): Pointer
    fun namespace_VectorTest_len(handle: Pointer): FFISizet
    fun namespace_VectorTest_get(handle: Pointer, idx: FFISizet): OptionDouble
    fun namespace_VectorTest_push(handle: Pointer, value: Double): Unit
}

class RenamedVectorTest internal constructor (
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

    private class RenamedVectorTestCleaner(val handle: Pointer, val lib: RenamedVectorTestLib) : Runnable {
        override fun run() {
            lib.namespace_VectorTest_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, RenamedVectorTest.RenamedVectorTestCleaner(handle, RenamedVectorTest.lib));
    }

    companion object {
        internal val libClass: Class<RenamedVectorTestLib> = RenamedVectorTestLib::class.java
        internal val lib: RenamedVectorTestLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(): RenamedVectorTest {
            
            val returnVal = lib.namespace_VectorTest_new();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = RenamedVectorTest(handle, selfEdges, true)
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