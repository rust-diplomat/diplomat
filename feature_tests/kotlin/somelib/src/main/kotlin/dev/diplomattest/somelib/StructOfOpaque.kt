package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface StructOfOpaqueLib: Library {
}

internal class StructOfOpaqueNative: Structure(), Structure.ByValue {
    @JvmField
    internal var i: Pointer = Pointer(0);
    @JvmField
    internal var j: Pointer = Pointer(0);

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("i", "j")
    }
}




internal class OptionStructOfOpaqueNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: StructOfOpaqueNative = StructOfOpaqueNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): StructOfOpaqueNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: StructOfOpaqueNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: StructOfOpaqueNative): OptionStructOfOpaqueNative {
            return OptionStructOfOpaqueNative(value, 1)
        }

        internal fun none(): OptionStructOfOpaqueNative {
            return OptionStructOfOpaqueNative(StructOfOpaqueNative(), 0)
        }
    }

}

class StructOfOpaque (var i: Opaque, var j: OpaqueMut) {
    companion object {

        internal val libClass: Class<StructOfOpaqueLib> = StructOfOpaqueLib::class.java
        internal val lib: StructOfOpaqueLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(StructOfOpaqueNative::class.java).toLong()

        internal fun fromNative(nativeStruct: StructOfOpaqueNative, aEdges: List<Any?>): StructOfOpaque {
            val i: Opaque = Opaque(nativeStruct.i, listOf(), false)
            val j: OpaqueMut = OpaqueMut(nativeStruct.j, listOf(), false)

            return StructOfOpaque(i, j)
        }

    }
    internal fun toNative(aAppendArray: Array<MutableList<Any>>): StructOfOpaqueNative {
        var native = StructOfOpaqueNative()
        native.i = this.i.handle
        native.j = this.j.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */
        return native
    }

    internal fun aEdges(): List<Any?> {
        return TODO("todo")
    }
}