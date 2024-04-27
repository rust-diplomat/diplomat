package dev.gigapixel.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface RefListParameterLib: Library {
    fun RefListParameter_destroy(handle: Long)
}

class RefListParameter internal constructor (
    internal val handle: Long,
    internal val selfEdges: List<Any>) {

    internal class RefListParameterCleaner(val handle: Long, val lib: RefListParameterLib) : Runnable {
        override fun run() {
            lib.RefListParameter_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<RefListParameterLib> = RefListParameterLib::class.java
        internal val lib: RefListParameterLib = Native.load("somelib", libClass)
    }

}
