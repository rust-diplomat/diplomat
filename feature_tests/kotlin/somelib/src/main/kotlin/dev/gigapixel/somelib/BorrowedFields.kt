package dev.gigapixel.somelib

import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BorrowedFieldsLib: Library {
}

class BorrowedFieldsNative: Structure(), Structure.ByValue {
    @JvmField
    var a: Slice = Slice();
    @JvmField
    var b: Slice = Slice();
    @JvmField
    var c: Slice = Slice();
  
    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a", "b", "c")
    }
}

class BorrowedFields internal constructor (
    internal val nativeStruct: BorrowedFieldsNative,
    internal val aEdges: List<Any>
    ) {
    val a: String = PrimitiveArrayTools.getUtf16(nativeStruct.a)
    val b: String = PrimitiveArrayTools.getUtf8(nativeStruct.b)
    val c: String = PrimitiveArrayTools.getUtf8(nativeStruct.c)

    companion object {
        internal val libClass: Class<BorrowedFieldsLib> = BorrowedFieldsLib::class.java
        internal val lib: BorrowedFieldsLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(BorrowedFieldsNative::class.java).toLong()
    }

}
