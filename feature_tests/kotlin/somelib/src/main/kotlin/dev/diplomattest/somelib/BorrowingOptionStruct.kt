package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BorrowingOptionStructLib: Library {
}

internal class BorrowingOptionStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: OptionSlice = OptionSlice.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a")
    }
}




internal class OptionBorrowingOptionStructNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: BorrowingOptionStructNative = BorrowingOptionStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): BorrowingOptionStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: BorrowingOptionStructNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: BorrowingOptionStructNative): OptionBorrowingOptionStructNative {
            return OptionBorrowingOptionStructNative(value, 1)
        }

        internal fun none(): OptionBorrowingOptionStructNative {
            return OptionBorrowingOptionStructNative(BorrowingOptionStructNative(), 0)
        }
    }

}

class BorrowingOptionStruct internal constructor (
    internal val nativeStruct: BorrowingOptionStructNative,
    internal val aEdges: List<Any?>
    ) {
    val a: String? = nativeStruct.a.option()?.let { PrimitiveArrayTools.getUtf8(it) }

    companion object {
        internal val libClass: Class<BorrowingOptionStructLib> = BorrowingOptionStructLib::class.java
        internal val lib: BorrowingOptionStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(BorrowingOptionStructNative::class.java).toLong()
    }

}
