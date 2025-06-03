package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BorrowedFieldsReturningLib: Library {
}

internal class BorrowedFieldsReturningNative: Structure(), Structure.ByValue {
    @JvmField
    internal var bytes: Slice = Slice();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("bytes")
    }
}

class BorrowedFieldsReturning internal constructor (
    internal val nativeStruct: BorrowedFieldsReturningNative,
    internal val aEdges: List<Any?>
    ) {
    val bytes: String = PrimitiveArrayTools.getUtf8(nativeStruct.bytes)

    companion object {
        internal val libClass: Class<BorrowedFieldsReturningLib> = BorrowedFieldsReturningLib::class.java
        internal val lib: BorrowedFieldsReturningLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(BorrowedFieldsReturningNative::class.java).toLong()
    }

}
