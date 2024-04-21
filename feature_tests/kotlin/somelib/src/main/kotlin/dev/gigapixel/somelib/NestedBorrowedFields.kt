package dev.gigapixel.somelib

import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface NestedBorrowedFieldsLib: Library {
}

class NestedBorrowedFieldsNative: Structure(), Structure.ByValue {
    @JvmField
    var fields: BorrowedFieldsNative = BorrowedFieldsNative();
    @JvmField
    var bounds: BorrowedFieldsWithBoundsNative = BorrowedFieldsWithBoundsNative();
    @JvmField
    var bounds2: BorrowedFieldsWithBoundsNative = BorrowedFieldsWithBoundsNative();
  
    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("fields", "bounds", "bounds2")
    }
}

class NestedBorrowedFields internal constructor (
    internal val nativeStruct: NestedBorrowedFieldsNative,
    internal val xEdges: List<Any>,
    internal val yEdges: List<Any>,
    internal val zEdges: List<Any>
    ) {
    val fields: BorrowedFields = BorrowedFields(nativeStruct.fields, listOf())
    val bounds: BorrowedFieldsWithBounds = BorrowedFieldsWithBounds(nativeStruct.bounds, listOf(), listOf(), listOf())
    val bounds2: BorrowedFieldsWithBounds = BorrowedFieldsWithBounds(nativeStruct.bounds2, listOf(), listOf(), listOf())

    companion object {
        internal val libClass: Class<NestedBorrowedFieldsLib> = NestedBorrowedFieldsLib::class.java
        internal val lib: NestedBorrowedFieldsLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(NestedBorrowedFieldsNative::class.java).toLong()
    }

}
