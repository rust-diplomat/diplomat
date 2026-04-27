package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OutTupleStructLib: Library {
    fun OutTupleStruct_new(): OutTupleStructNative
}

internal class OutTupleStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var x: Int = 0;
    @JvmField
    internal var y: Int = 0;
    @JvmField
    internal var primitive: PrimitiveStructNative = PrimitiveStructNative();
    @JvmField
    internal var opaque: Pointer = Pointer(0);

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("x", "y", "primitive", "opaque")
    }
}




internal class OptionOutTupleStructNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: OutTupleStructNative = OutTupleStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): OutTupleStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: OutTupleStructNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: OutTupleStructNative): OptionOutTupleStructNative {
            return OptionOutTupleStructNative(value, 1)
        }

        internal fun none(): OptionOutTupleStructNative {
            return OptionOutTupleStructNative(OutTupleStructNative(), 0)
        }
    }

}

class OutTupleStruct (var x: Int, var y: Int, var primitive: PrimitiveStruct, var opaque: Opaque) {
    companion object {

        internal val libClass: Class<OutTupleStructLib> = OutTupleStructLib::class.java
        internal val lib: OutTupleStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(OutTupleStructNative::class.java).toLong()

        internal fun fromNative(nativeStruct: OutTupleStructNative): OutTupleStruct {
            val x: Int = nativeStruct.x
            val y: Int = nativeStruct.y
            val primitive: PrimitiveStruct = PrimitiveStruct.fromNative(nativeStruct.primitive)
            val opaque: Opaque = Opaque(nativeStruct.opaque, listOf(), true)

            return OutTupleStruct(x, y, primitive, opaque)
        }

        @JvmStatic
        
        fun new_(): OutTupleStruct {
            
            val returnVal = lib.OutTupleStruct_new();
            val returnStruct = OutTupleStruct.fromNative(returnVal)
            return returnStruct
        }
    }
}