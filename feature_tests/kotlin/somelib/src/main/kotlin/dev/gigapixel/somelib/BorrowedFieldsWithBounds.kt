package dev.gigapixel.somelib

import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BorrowedFieldsWithBoundsLib: Library {
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
    internal val aEdges: List<Any>,
    internal val bEdges: List<Any>,
    internal val cEdges: List<Any>
    ) {
    val fieldA: String = PrimitiveArrayTools.getUtf16(nativeStruct.fieldA)
    val fieldB: String = PrimitiveArrayTools.getUtf8(nativeStruct.fieldB)
    val fieldC: String = PrimitiveArrayTools.getUtf8(nativeStruct.fieldC)

    companion object {
        internal val libClass: Class<BorrowedFieldsWithBoundsLib> = BorrowedFieldsWithBoundsLib::class.java
        internal val lib: BorrowedFieldsWithBoundsLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(BorrowedFieldsWithBoundsNative::class.java).toLong()
    }

}
