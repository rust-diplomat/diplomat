package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BorrowedFieldsLib: Library {
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




internal class OptionBorrowedFieldsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: BorrowedFieldsNative = BorrowedFieldsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): BorrowedFieldsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: BorrowedFieldsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: BorrowedFieldsNative): OptionBorrowedFieldsNative {
            return OptionBorrowedFieldsNative(value, 1)
        }

        internal fun none(): OptionBorrowedFieldsNative {
            return OptionBorrowedFieldsNative(BorrowedFieldsNative(), 0)
        }
    }

}

class BorrowedFields (var a: String, var b: String, var c: String) {
    companion object {

        internal val libClass: Class<BorrowedFieldsLib> = BorrowedFieldsLib::class.java
        internal val lib: BorrowedFieldsLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(BorrowedFieldsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: BorrowedFieldsNative, aEdges: List<Any?>): BorrowedFields {
            val a: String = PrimitiveArrayTools.getUtf16(nativeStruct.a)
            val b: String = PrimitiveArrayTools.getUtf8(nativeStruct.b)
            val c: String = PrimitiveArrayTools.getUtf8(nativeStruct.c)

            return BorrowedFields(a, b, c)
        }

    }
    internal fun toNative(): BorrowedFieldsNative {
        var native = BorrowedFieldsNative()
        native.a = PrimitiveArrayTools.borrowUtf16(this.a).slice
        native.b = PrimitiveArrayTools.borrowUtf8(this.b).slice
        native.c = PrimitiveArrayTools.borrowUtf8(this.c).slice
        return native
    }

    internal fun aEdges(): List<Any?> {
        return TODO("todo")
    }
}