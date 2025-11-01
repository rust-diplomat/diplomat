package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface AttrOpaque2Lib: Library {
    fun namespace_AttrOpaque2_destroy(handle: Pointer)
}

class AttrOpaque2 internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class AttrOpaque2Cleaner(val handle: Pointer, val lib: AttrOpaque2Lib) : Runnable {
        override fun run() {
            lib.namespace_AttrOpaque2_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<AttrOpaque2Lib> = AttrOpaque2Lib::class.java
        internal val lib: AttrOpaque2Lib = Native.load("somelib", libClass)
    }

}