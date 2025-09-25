package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface NestedBorrowedFieldsLib: Library {
    fun NestedBorrowedFields_from_bar_and_foo_and_strings(bar: Pointer, foo: Pointer, dstr16X: Slice, dstr16Z: Slice, utf8StrY: Slice, utf8StrZ: Slice): NestedBorrowedFieldsNative
}

internal class NestedBorrowedFieldsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var fields: BorrowedFieldsNative = BorrowedFieldsNative();
    @JvmField
    internal var bounds: BorrowedFieldsWithBoundsNative = BorrowedFieldsWithBoundsNative();
    @JvmField
    internal var bounds2: BorrowedFieldsWithBoundsNative = BorrowedFieldsWithBoundsNative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("fields", "bounds", "bounds2")
    }
}

class NestedBorrowedFields internal constructor (
    internal val nativeStruct: NestedBorrowedFieldsNative,
    internal val xEdges: List<Any?>,
    internal val yEdges: List<Any?>,
    internal val zEdges: List<Any?>
    ) {
    val fields: BorrowedFields = BorrowedFields(nativeStruct.fields, xEdges)
    val bounds: BorrowedFieldsWithBounds = BorrowedFieldsWithBounds(nativeStruct.bounds, xEdges, yEdges, yEdges)
    val bounds2: BorrowedFieldsWithBounds = BorrowedFieldsWithBounds(nativeStruct.bounds2, zEdges, zEdges, zEdges)

    companion object {
        internal val libClass: Class<NestedBorrowedFieldsLib> = NestedBorrowedFieldsLib::class.java
        internal val lib: NestedBorrowedFieldsLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(NestedBorrowedFieldsNative::class.java).toLong()
        @JvmStatic
        
        fun fromBarAndFooAndStrings(bar: Bar, foo: Foo, dstr16X: String, dstr16Z: String, utf8StrY: String, utf8StrZ: String): NestedBorrowedFields {
            val (dstr16XMem, dstr16XSlice) = PrimitiveArrayTools.borrowUtf16(dstr16X)
            val (dstr16ZMem, dstr16ZSlice) = PrimitiveArrayTools.borrowUtf16(dstr16Z)
            val (utf8StrYMem, utf8StrYSlice) = PrimitiveArrayTools.borrowUtf8(utf8StrY)
            val (utf8StrZMem, utf8StrZSlice) = PrimitiveArrayTools.borrowUtf8(utf8StrZ)
            
            val returnVal = lib.NestedBorrowedFields_from_bar_and_foo_and_strings(bar.handle, foo.handle, dstr16XSlice, dstr16ZSlice, utf8StrYSlice, utf8StrZSlice);
            
            val xEdges: List<Any?> = listOf(bar) + listOf(dstr16XMem) + listOf(utf8StrYMem)
            val yEdges: List<Any?> = listOf(bar) + listOf(utf8StrYMem)
            val zEdges: List<Any?> = listOf(foo) + listOf(dstr16ZMem) + listOf(utf8StrZMem)
            val returnStruct = NestedBorrowedFields(returnVal, xEdges, yEdges, zEdges)
            return returnStruct
        }
    }

}
