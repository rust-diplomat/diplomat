package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure


internal interface NestedLib: Library {
    fun namespace_Nested_destroy(handle: Pointer)
}

class Nested internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class NestedCleaner(val handle: Pointer, val lib: NestedLib) : Runnable {
        override fun run() {
            lib.namespace_Nested_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<NestedLib> = NestedLib::class.java
        internal val lib: NestedLib = Native.load("somelib", libClass)
    }

}
