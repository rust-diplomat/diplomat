package dev.diplomattest.somelib

import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BorrowedFieldsWithBoundsLib: Library {
    fun BorrowedFieldsWithBounds_from_foo_and_strings(foo: Pointer, dstr16X: Slice, utf8StrZ: Slice): BorrowedFieldsWithBoundsNative
}

class BorrowedFieldsWithBoundsNative: Structure(), Structure.ByValue {
    @JvmField
    var fieldA: Slice = Slice();
    @JvmField
    var fieldB: Slice = Slice();
    @JvmField
    var fieldC: Slice = Slice();
  
    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("fieldA", "fieldB", "fieldC")
    }
}

class BorrowedFieldsWithBounds internal constructor (
    internal val nativeStruct: BorrowedFieldsWithBoundsNative,
    internal val bEdges: List<Any>,
    internal val aEdges: List<Any>,
    internal val cEdges: List<Any>
    ) {
    val fieldA: String = PrimitiveArrayTools.getUtf16(nativeStruct.fieldA)
    val fieldB: String = PrimitiveArrayTools.getUtf8(nativeStruct.fieldB)
    val fieldC: String = PrimitiveArrayTools.getUtf8(nativeStruct.fieldC)

    companion object {
        internal val libClass: Class<BorrowedFieldsWithBoundsLib> = BorrowedFieldsWithBoundsLib::class.java
        internal val lib: BorrowedFieldsWithBoundsLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(BorrowedFieldsWithBoundsNative::class.java).toLong()
        fun fromFooAndStrings(foo: Foo, dstr16X: String, utf8StrZ: String): BorrowedFieldsWithBounds {
            val (dstr16XMem, dstr16XSlice) = PrimitiveArrayTools.readUtf16(dstr16X)
            val (utf8StrZMem, utf8StrZSlice) = PrimitiveArrayTools.readUtf8(utf8StrZ)
            
            val returnVal = lib.BorrowedFieldsWithBounds_from_foo_and_strings(foo.handle, dstr16XSlice, utf8StrZSlice);
        
            val xEdges: List<Any> = listOf(foo) + listOf(dstr16XMem) + listOf(utf8StrZMem)
            val yEdges: List<Any> = listOf(foo) + listOf(utf8StrZMem)
            val zEdges: List<Any> = listOf(utf8StrZMem)
            val returnStruct = BorrowedFieldsWithBounds(returnVal, xEdges, yEdges, zEdges)
            return returnStruct
        
        }
    }

}
