package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure


internal interface RefListLib: Library {
    fun RefList_destroy(handle: Pointer)
    fun RefList_node(data: Pointer): Pointer
}

class RefList internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any>,
)  {

    internal class RefListCleaner(val handle: Pointer, val lib: RefListLib) : Runnable {
        override fun run() {
            lib.RefList_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<RefListLib> = RefListLib::class.java
        internal val lib: RefListLib = Native.load("somelib", libClass)
        
        fun node(data: RefListParameter): RefList {
            
            val returnVal = lib.RefList_node(data.handle);
            val selfEdges: List<Any> = listOf()
            val bEdges: List<Any> = listOf(data)
            val handle = returnVal 
            val returnOpaque = RefList(handle, selfEdges, bEdges)
            CLEANER.register(returnOpaque, RefList.RefListCleaner(handle, RefList.lib));
            return returnOpaque
        }
    }

}
