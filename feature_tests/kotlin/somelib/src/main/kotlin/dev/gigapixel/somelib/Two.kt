package dev.gigapixel.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


interface TwoLib: Library {
    fun Two_destroy(handle: Long)
}

class Two internal constructor (
    internal val handle: Long,
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any>,
    internal val bEdges: List<Any>,
    ) {

    internal class TwoCleaner(val handle: Long, val lib: TwoLib) : Runnable {
        override fun run() {
            lib.Two_destroy(handle)
        }
    }

    companion object {
        val libClass: Class<TwoLib> = TwoLib::class.java
        val lib: TwoLib = Native.load("somelib", libClass)
    }

}
