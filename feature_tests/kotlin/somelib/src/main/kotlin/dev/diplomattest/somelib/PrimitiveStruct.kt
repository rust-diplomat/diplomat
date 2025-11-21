package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PrimitiveStructLib: Library {
}

internal class PrimitiveStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var x: Float = 0.0F;
    @JvmField
    internal var a: Byte = 0;
    @JvmField
    internal var b: Int = 0;
    @JvmField
    internal var c: Long = 0;
    @JvmField
    internal var d: FFIIsizet = FFIIsizet();
    @JvmField
    internal var e: Byte = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("x", "a", "b", "c", "d", "e")
    }
}




internal class OptionPrimitiveStructNative: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: PrimitiveStructNative = PrimitiveStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): PrimitiveStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}

class PrimitiveStruct internal constructor (
    internal val nativeStruct: PrimitiveStructNative) {
    val x: Float = nativeStruct.x
    val a: Boolean = nativeStruct.a > 0
    val b: Int = nativeStruct.b
    val c: Long = nativeStruct.c
    val d: Long = nativeStruct.d.toLong()
    val e: Byte = nativeStruct.e

    companion object {
        internal val libClass: Class<PrimitiveStructLib> = PrimitiveStructLib::class.java
        internal val lib: PrimitiveStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(PrimitiveStructNative::class.java).toLong()
    }

}
