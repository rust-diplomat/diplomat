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




internal class OptionBorrowedFieldsReturningNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: BorrowedFieldsReturningNative = BorrowedFieldsReturningNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): BorrowedFieldsReturningNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: BorrowedFieldsReturningNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: BorrowedFieldsReturningNative): OptionBorrowedFieldsReturningNative {
            return OptionBorrowedFieldsReturningNative(value, 1)
        }

        internal fun none(): OptionBorrowedFieldsReturningNative {
            return OptionBorrowedFieldsReturningNative(BorrowedFieldsReturningNative(), 0)
        }
    }

}

class BorrowedFieldsReturning internal constructor (
    internal val nativeStruct: BorrowedFieldsReturningNative,
    internal val aEdges: List<Any?>
    ) {
    val bytes: String = PrimitiveArrayTools.getUtf8(nativeStruct.bytes)

    companion object {
        internal val libClass: Class<BorrowedFieldsReturningLib> = BorrowedFieldsReturningLib::class.java
        internal val lib: BorrowedFieldsReturningLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(BorrowedFieldsReturningNative::class.java).toLong()
    }

}
