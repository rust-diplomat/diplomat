package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BarLib: Library {
    fun Bar_destroy(handle: Pointer)
    fun Bar_foo(handle: Pointer): Pointer
}

class Bar internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val bEdges: List<Any?>,
    internal val aEdges: List<Any?>,
)  {

    internal class BarCleaner(val handle: Pointer, val lib: BarLib) : Runnable {
        override fun run() {
            lib.Bar_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<BarLib> = BarLib::class.java
        internal val lib: BarLib = Native.load("somelib", libClass)
    }
    
    fun foo(): Foo {
        
        val returnVal = lib.Bar_foo(handle);
        val selfEdges: List<Any> = listOf(this)
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = Foo(handle, selfEdges, aEdges)
        return returnOpaque
    }

}