package dev.gigapixel.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface Utf16WrapLib: Library {
    fun Utf16Wrap_destroy(handle: Long)
    fun Utf16Wrap_borrow_cont(handle: Long): Slice
    fun Utf16Wrap_owned(handle: Long): Slice
}

class Utf16Wrap internal constructor (
    internal val handle: Long,

    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>) {

    internal class Utf16WrapCleaner(val handle: Long, val lib: Utf16WrapLib) : Runnable {
        override fun run() {
            lib.Utf16Wrap_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<Utf16WrapLib> = Utf16WrapLib::class.java
        internal val lib: Utf16WrapLib = Native.load("somelib", libClass)
    }
    fun borrowCont(): String {
        
        val returnVal = lib.Utf16Wrap_borrow_cont(handle);
        return PrimitiveArrayTools.getUtf16(returnVal)
    }
    fun owned(): String {
        
        val returnVal = lib.Utf16Wrap_owned(handle);
        val string = PrimitiveArrayTools.getUtf16(returnVal)
        Native.free(Pointer.nativeValue(returnVal.data))
        return string
    }

}
