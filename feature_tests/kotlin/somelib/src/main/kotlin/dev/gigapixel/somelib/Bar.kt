package dev.gigapixel.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface BarLib: Library {
    fun Bar_destroy(handle: Long)
    fun Bar_foo(handle: Long): Long
}

class Bar internal constructor (
    internal val handle: Long,
    internal val selfEdges: List<Any>,
    internal val bEdges: List<Any>,
    internal val aEdges: List<Any>,
    ) {

    internal class BarCleaner(val handle: Long, val lib: BarLib) : Runnable {
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
        val aEdges: List<Any> = listOf(this)
        val handle = returnVal
        val returnOpaque = Foo(handle, selfEdges, aEdges)
        
        return returnOpaque
    
    }

}
