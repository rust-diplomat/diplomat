package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure


internal interface Nested2Lib: Library {
    fun namespace_Nested2_destroy(handle: Pointer)
}

class Nested2 internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class Nested2Cleaner(val handle: Pointer, val lib: Nested2Lib) : Runnable {
        override fun run() {
            lib.namespace_Nested2_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<Nested2Lib> = Nested2Lib::class.java
        internal val lib: Nested2Lib = Native.load("somelib", libClass)
    }

}
