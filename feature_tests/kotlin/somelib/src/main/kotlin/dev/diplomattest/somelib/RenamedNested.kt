package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedNestedLib: Library {
    fun namespace_Nested_destroy(handle: Pointer)
}

class RenamedNested internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class RenamedNestedCleaner(val handle: Pointer, val lib: RenamedNestedLib) : Runnable {
        override fun run() {
            lib.namespace_Nested_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<RenamedNestedLib> = RenamedNestedLib::class.java
        internal val lib: RenamedNestedLib = Native.load("diplomat_feature_tests", libClass)
    }

}