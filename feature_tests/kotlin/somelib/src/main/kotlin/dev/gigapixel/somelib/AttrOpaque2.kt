package dev.gigapixel.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface AttrOpaque2Lib: Library {
    fun AttrOpaque2_destroy(handle: Long)
}

class AttrOpaque2 internal constructor (
    internal val handle: Long,
    internal val selfEdges: List<Any>) {

    internal class AttrOpaque2Cleaner(val handle: Long, val lib: AttrOpaque2Lib) : Runnable {
        override fun run() {
            lib.AttrOpaque2_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<AttrOpaque2Lib> = AttrOpaque2Lib::class.java
        internal val lib: AttrOpaque2Lib = Native.load("somelib", libClass)
    }

}
