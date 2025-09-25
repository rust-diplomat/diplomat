package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface StructWithSlicesLib: Library {
    fun StructWithSlices_return_last(nativeStruct: StructWithSlicesNative, write: Pointer): Unit
}

internal class StructWithSlicesNative: Structure(), Structure.ByValue {
    @JvmField
    internal var first: Slice = Slice();
    @JvmField
    internal var second: Slice = Slice();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("first", "second")
    }
}

class StructWithSlices internal constructor (
    internal val nativeStruct: StructWithSlicesNative,
    internal val aEdges: List<Any?>
    ) {
    val first: String = PrimitiveArrayTools.getUtf8(nativeStruct.first)
    val second: UShortArray = PrimitiveArrayTools.getUShortArray(nativeStruct.second)

    companion object {
        internal val libClass: Class<StructWithSlicesLib> = StructWithSlicesLib::class.java
        internal val lib: StructWithSlicesLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(StructWithSlicesNative::class.java).toLong()
    }
    
    fun returnLast(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.StructWithSlices_return_last(nativeStruct, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}
