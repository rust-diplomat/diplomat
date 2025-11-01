package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MyIteratorLib: Library {
    fun namespace_MyIterator_destroy(handle: Pointer)
    fun namespace_MyIterator_next(handle: Pointer): OptionFFIUint8
}
typealias MyIteratorIteratorItem = UByte

class MyIterator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
): Iterator<UByte> {

    internal class MyIteratorCleaner(val handle: Pointer, val lib: MyIteratorLib) : Runnable {
        override fun run() {
            lib.namespace_MyIterator_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<MyIteratorLib> = MyIteratorLib::class.java
        internal val lib: MyIteratorLib = Native.load("somelib", libClass)
    }
    
    internal fun nextInternal(): UByte? {
        
        val returnVal = lib.namespace_MyIterator_next(handle);
        return returnVal.option()?.toUByte()
    }

    var iterVal = nextInternal()

    override fun hasNext(): Boolean {
       return iterVal != null
    }

    override fun next(): UByte{
        val returnVal = iterVal
        if (returnVal == null) {
            throw NoSuchElementException()
        } else {
            iterVal = nextInternal()
            return returnVal
        }
    }

}