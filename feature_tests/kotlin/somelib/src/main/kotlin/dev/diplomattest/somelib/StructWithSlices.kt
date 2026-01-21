package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface StructWithSlicesLib: Library {
    fun StructWithSlices_return_last(nativeStruct: StructWithSlicesNative, write: Pointer): Unit
}

internal class StructWithSlicesNative: Structure(), Structure.ByValue {
    @JvmField
    internal var first: Slice = Slice();
    @JvmField
    internal var second: Slice = Slice();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("first", "second")
    }
}




internal class OptionStructWithSlicesNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: StructWithSlicesNative = StructWithSlicesNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): StructWithSlicesNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: StructWithSlicesNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: StructWithSlicesNative): OptionStructWithSlicesNative {
            return OptionStructWithSlicesNative(value, 1)
        }

        internal fun none(): OptionStructWithSlicesNative {
            return OptionStructWithSlicesNative(StructWithSlicesNative(), 0)
        }
    }

}

class StructWithSlices (var first: String, var second: UShortArray) {
    companion object {

        internal val libClass: Class<StructWithSlicesLib> = StructWithSlicesLib::class.java
        internal val lib: StructWithSlicesLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(StructWithSlicesNative::class.java).toLong()

        internal fun fromNative(nativeStruct: StructWithSlicesNative, aEdges: List<Any?>): StructWithSlices {
            val first: String = PrimitiveArrayTools.getUtf8(nativeStruct.first)
            val second: UShortArray = PrimitiveArrayTools.getUShortArray(nativeStruct.second)

            return StructWithSlices(first, second)
        }

    }
    internal fun toNative(aAppendArray: Array<MutableList<Any>>): StructWithSlicesNative {
        var native = StructWithSlicesNative()
        native.first = PrimitiveArrayTools.borrowUtf8(this.first).slice
        native.second = PrimitiveArrayTools.borrow(this.second).slice
        return native
    }

    internal fun aEdges(): List<Any?> {
        return TODO("todo")
    }
    
    fun returnLast(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.StructWithSlices_return_last(this.toNative(), write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
}