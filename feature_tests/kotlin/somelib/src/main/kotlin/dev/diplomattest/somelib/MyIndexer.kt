package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MyIndexerLib: Library {
    fun namespace_MyIndexer_destroy(handle: Pointer)
    fun namespace_MyIndexer_get(handle: Pointer, i: FFISizet): OptionSlice
}

class MyIndexer internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class MyIndexerCleaner(val handle: Pointer, val lib: MyIndexerLib) : Runnable {
        override fun run() {
            lib.namespace_MyIndexer_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<MyIndexerLib> = MyIndexerLib::class.java
        internal val lib: MyIndexerLib = Native.load("somelib", libClass)
    }
    
    internal fun getInternal(i: ULong): String? {
        
        val returnVal = lib.namespace_MyIndexer_get(handle, FFISizet(i));
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }

    operator fun get(index: ULong): String {
        val returnVal = getInternal(index)
        if (returnVal == null) {
            throw IndexOutOfBoundsException("Index $index is out of bounds.")
        } else {
            return returnVal
        }
    }

}