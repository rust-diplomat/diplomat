package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ContainingTupleLib: Library {
}

internal class ContainingTupleNative: Structure(), Structure.ByValue {
    @JvmField
    internal var inner: TupleStructNative = TupleStructNative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("inner")
    }
}




internal class OptionContainingTupleNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: ContainingTupleNative = ContainingTupleNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): ContainingTupleNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: ContainingTupleNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: ContainingTupleNative): OptionContainingTupleNative {
            return OptionContainingTupleNative(value, 1)
        }

        internal fun none(): OptionContainingTupleNative {
            return OptionContainingTupleNative(ContainingTupleNative(), 0)
        }
    }

}

class ContainingTuple (var inner: TupleStruct) {
    companion object {

        internal val libClass: Class<ContainingTupleLib> = ContainingTupleLib::class.java
        internal val lib: ContainingTupleLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(ContainingTupleNative::class.java).toLong()

        internal fun fromNative(nativeStruct: ContainingTupleNative, aEdges: List<Any?>): ContainingTuple {
            val inner: TupleStruct = TupleStruct.fromNative(nativeStruct.inner, aEdges)

            return ContainingTuple(inner)
        }

    }
    internal fun toNative(aAppendArray: Array<MutableList<Any>>): ContainingTupleNative {
        var native = ContainingTupleNative()
        native.inner = this.inner.toNative(aAppendArray = aAppendArray)
        return native
    }

    internal fun aEdges(): List<Any?> {
        return TODO("todo")
    }
}