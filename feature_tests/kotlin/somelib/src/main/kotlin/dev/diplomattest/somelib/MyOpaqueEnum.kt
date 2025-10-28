package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MyOpaqueEnumLib: Library {
    fun MyOpaqueEnum_destroy(handle: Pointer)
    fun MyOpaqueEnum_new(): Pointer
    fun MyOpaqueEnum_to_string(handle: Pointer, write: Pointer): Unit
}

class MyOpaqueEnum internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class MyOpaqueEnumCleaner(val handle: Pointer, val lib: MyOpaqueEnumLib) : Runnable {
        override fun run() {
            lib.MyOpaqueEnum_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<MyOpaqueEnumLib> = MyOpaqueEnumLib::class.java
        internal val lib: MyOpaqueEnumLib = Native.load("somelib", libClass)
        @JvmStatic
        
        fun new_(): MyOpaqueEnum {
            
            val returnVal = lib.MyOpaqueEnum_new();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MyOpaqueEnum(handle, selfEdges)
            CLEANER.register(returnOpaque, MyOpaqueEnum.MyOpaqueEnumCleaner(handle, MyOpaqueEnum.lib));
            return returnOpaque
        }
    }
    
    override fun toString(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.MyOpaqueEnum_to_string(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}