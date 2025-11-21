package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OptionInputStructLib: Library {
    fun OptionInputStruct_new_from_parts(a: OptionFFIUint8, b: OptionInt, c: OptionInt): OptionInputStructNative
}

internal class OptionInputStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: OptionFFIUint8 = OptionFFIUint8.none();
    @JvmField
    internal var b: OptionInt = OptionInt.none();
    @JvmField
    internal var c: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a", "b", "c")
    }
}




internal class OptionOptionInputStructNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: OptionInputStructNative = OptionInputStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): OptionInputStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: OptionInputStructNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: OptionInputStructNative): OptionOptionInputStructNative {
            return OptionOptionInputStructNative(value, 1)
        }

        internal fun none(): OptionOptionInputStructNative {
            return OptionOptionInputStructNative(OptionInputStructNative(), 0)
        }
    }

}

class OptionInputStruct internal constructor (
    internal val nativeStruct: OptionInputStructNative) {
    val a: UByte? = nativeStruct.a.option()?.let { it.toUByte() }
    val b: Int? = nativeStruct.b.option()?.let { it }
    val c: OptionEnum? = nativeStruct.c.option()?.let { OptionEnum.fromNative(it) }

    companion object {
        internal val libClass: Class<OptionInputStructLib> = OptionInputStructLib::class.java
        internal val lib: OptionInputStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(OptionInputStructNative::class.java).toLong()
        @JvmStatic
        
        /** Needed until https://github.com/rust-diplomat/diplomat/issues/1001 is fixed
        */
        fun newFromParts(a: UByte?, b: Int?, c: OptionEnum?): OptionInputStruct {
            
            val returnVal = lib.OptionInputStruct_new_from_parts(a?.let { OptionFFIUint8.some(FFIUint8(it)) } ?: OptionFFIUint8.none(), b?.let { OptionInt.some(it) } ?: OptionInt.none(), c?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            
            val returnStruct = OptionInputStruct(returnVal)
            return returnStruct
        }
    }

}
