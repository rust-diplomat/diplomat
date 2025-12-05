package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BorrowedFieldsWithBoundsLib: Library {
}

internal class BorrowedFieldsWithBoundsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var fieldA: Slice = Slice();
    @JvmField
    internal var fieldB: Slice = Slice();
    @JvmField
    internal var fieldC: Slice = Slice();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("fieldA", "fieldB", "fieldC")
    }
}




internal class OptionBorrowedFieldsWithBoundsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: BorrowedFieldsWithBoundsNative = BorrowedFieldsWithBoundsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): BorrowedFieldsWithBoundsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: BorrowedFieldsWithBoundsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: BorrowedFieldsWithBoundsNative): OptionBorrowedFieldsWithBoundsNative {
            return OptionBorrowedFieldsWithBoundsNative(value, 1)
        }

        internal fun none(): OptionBorrowedFieldsWithBoundsNative {
            return OptionBorrowedFieldsWithBoundsNative(BorrowedFieldsWithBoundsNative(), 0)
        }
    }

}

class BorrowedFieldsWithBounds (var fieldA: String, var fieldB: String, var fieldC: String) {
    companion object {

        internal val libClass: Class<BorrowedFieldsWithBoundsLib> = BorrowedFieldsWithBoundsLib::class.java
        internal val lib: BorrowedFieldsWithBoundsLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(BorrowedFieldsWithBoundsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: BorrowedFieldsWithBoundsNative, aEdges: List<Any?>, bEdges: List<Any?>, cEdges: List<Any?>): BorrowedFieldsWithBounds {
            val fieldA: String = PrimitiveArrayTools.getUtf16(nativeStruct.fieldA)
            val fieldB: String = PrimitiveArrayTools.getUtf8(nativeStruct.fieldB)
            val fieldC: String = PrimitiveArrayTools.getUtf8(nativeStruct.fieldC)

            return BorrowedFieldsWithBounds(fieldA, fieldB, fieldC)
        }

    }
    internal fun toNative(): BorrowedFieldsWithBoundsNative {
        var native = BorrowedFieldsWithBoundsNative()
        native.fieldA = PrimitiveArrayTools.borrowUtf16(this.fieldA).second
        native.fieldB = PrimitiveArrayTools.borrowUtf8(this.fieldB).second
        native.fieldC = PrimitiveArrayTools.borrowUtf8(this.fieldC).second
        return native
    }

    internal fun aEdges(): List<Any?> {
        return TODO("todo")
    }
    internal fun bEdges(): List<Any?> {
        return TODO("todo")
    }
    internal fun cEdges(): List<Any?> {
        return TODO("todo")
    }
}