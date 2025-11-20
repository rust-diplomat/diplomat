package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MyIterableLib: Library {
    fun namespace_MyIterable_destroy(handle: Pointer)
    fun namespace_MyIterable_new(x: Slice): Pointer
    fun namespace_MyIterable_iter(handle: Pointer): Pointer
}

class MyIterable internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
): Iterable<MyIteratorIteratorItem> {

    internal class MyIterableCleaner(val handle: Pointer, val lib: MyIterableLib) : Runnable {
        override fun run() {
            lib.namespace_MyIterable_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<MyIterableLib> = MyIterableLib::class.java
        internal val lib: MyIterableLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(x: UByteArray): MyIterable {
            val (xMem, xSlice) = PrimitiveArrayTools.borrow(x)
            
            val returnVal = lib.namespace_MyIterable_new(xSlice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MyIterable(handle, selfEdges)
            CLEANER.register(returnOpaque, MyIterable.MyIterableCleaner(handle, MyIterable.lib));
            if (xMem != null) xMem.close()
            return returnOpaque
        }
    }
    
    override fun iterator(): MyIterator {
        
        val returnVal = lib.namespace_MyIterable_iter(handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = MyIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, MyIterator.MyIteratorCleaner(handle, MyIterator.lib));
        return returnOpaque
    }

}