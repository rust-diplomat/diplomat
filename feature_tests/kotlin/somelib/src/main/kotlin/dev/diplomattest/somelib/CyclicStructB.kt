package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CyclicStructBLib: Library {
    fun CyclicStructB_get_a(): CyclicStructANative
    fun CyclicStructB_get_a_option(): OptionCyclicStructANative
}

internal class CyclicStructBNative: Structure(), Structure.ByValue {
    @JvmField
    internal var field: FFIUint8 = FFIUint8();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("field")
    }
}




internal class OptionCyclicStructBNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: CyclicStructBNative = CyclicStructBNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): CyclicStructBNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: CyclicStructBNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: CyclicStructBNative): OptionCyclicStructBNative {
            return OptionCyclicStructBNative(value, 1)
        }

        internal fun none(): OptionCyclicStructBNative {
            return OptionCyclicStructBNative(CyclicStructBNative(), 0)
        }
    }

}

class CyclicStructB (var field: UByte) {
    companion object {

        internal val libClass: Class<CyclicStructBLib> = CyclicStructBLib::class.java
        internal val lib: CyclicStructBLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CyclicStructBNative::class.java).toLong()

        internal fun fromNative(nativeStruct: CyclicStructBNative): CyclicStructB {
            val field: UByte = nativeStruct.field.toUByte()

            return CyclicStructB(field)
        }

        @JvmStatic
        
        fun getA(): CyclicStructA {
            
            val returnVal = lib.CyclicStructB_get_a();
            
            val returnStruct = CyclicStructA.fromNative(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun getAOption(): CyclicStructA? {
            
            val returnVal = lib.CyclicStructB_get_a_option();
            
            val intermediateOption = returnVal.option() ?: return null

            val returnStruct = CyclicStructA.fromNative(intermediateOption)
            return returnStruct
                                    
        }
    }
    internal fun toNative(): CyclicStructBNative {
        var native = CyclicStructBNative()
        native.field = FFIUint8(this.field)
        return native
    }

}