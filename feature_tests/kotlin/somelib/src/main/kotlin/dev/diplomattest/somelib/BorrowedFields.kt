package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BorrowedFieldsLib: Library {
    fun BorrowedFields_from_bar_and_strings(bar: Pointer, dstr16: Slice, utf8Str: Slice): BorrowedFieldsNative
}

internal class BorrowedFieldsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: Slice = Slice();
    @JvmField
    internal var b: Slice = Slice();
    @JvmField
    internal var c: Slice = Slice();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a", "b", "c")
    }
}

class BorrowedFields internal constructor (
    internal val nativeStruct: BorrowedFieldsNative,
    internal val aEdges: List<Any?>
    ) {
    val a: String = PrimitiveArrayTools.getUtf16(nativeStruct.a)
    val b: String = PrimitiveArrayTools.getUtf8(nativeStruct.b)
    val c: String = PrimitiveArrayTools.getUtf8(nativeStruct.c)

    companion object {
        internal val libClass: Class<BorrowedFieldsLib> = BorrowedFieldsLib::class.java
        internal val lib: BorrowedFieldsLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(BorrowedFieldsNative::class.java).toLong()
        @JvmStatic
        
        fun fromBarAndStrings(bar: Bar, dstr16: String, utf8Str: String): BorrowedFields {
            val (dstr16Mem, dstr16Slice) = PrimitiveArrayTools.borrowUtf16(dstr16)
            val (utf8StrMem, utf8StrSlice) = PrimitiveArrayTools.borrowUtf8(utf8Str)
            
            val returnVal = lib.BorrowedFields_from_bar_and_strings(bar.handle, dstr16Slice, utf8StrSlice);
            
            val xEdges: List<Any?> = listOf(bar) + listOf(dstr16Mem) + listOf(utf8StrMem)
            val returnStruct = BorrowedFields(returnVal, xEdges)
            return returnStruct
        }
    }

}
