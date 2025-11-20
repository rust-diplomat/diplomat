package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TestOpaqueLib: Library {
    fun namespace_TestOpaque_destroy(handle: Pointer)
}

class TestOpaque internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class TestOpaqueCleaner(val handle: Pointer, val lib: TestOpaqueLib) : Runnable {
        override fun run() {
            lib.namespace_TestOpaque_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<TestOpaqueLib> = TestOpaqueLib::class.java
        internal val lib: TestOpaqueLib = Native.load("diplomat_feature_tests", libClass)
    }

}