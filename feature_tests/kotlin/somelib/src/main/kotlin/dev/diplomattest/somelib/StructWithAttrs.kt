package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface StructWithAttrsLib: Library {
    fun namespace_StructWithAttrs_new_fallible(a: Boolean, b: FFIUint32): ResultStructWithAttrsNativeUnit
    fun namespace_StructWithAttrs_c(nativeStruct: StructWithAttrsNative): FFIUint32
    fun namespace_StructWithAttrs_deprecated(nativeStruct: StructWithAttrsNative): Unit
}

internal class StructWithAttrsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: Byte = 0;
    @JvmField
    internal var b: FFIUint32 = FFIUint32();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a", "b")
    }
}




internal class OptionStructWithAttrsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: StructWithAttrsNative = StructWithAttrsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): StructWithAttrsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: StructWithAttrsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: StructWithAttrsNative): OptionStructWithAttrsNative {
            return OptionStructWithAttrsNative(value, 1)
        }

        internal fun none(): OptionStructWithAttrsNative {
            return OptionStructWithAttrsNative(StructWithAttrsNative(), 0)
        }
    }

}

class StructWithAttrs internal constructor (
    internal val nativeStruct: StructWithAttrsNative) {
    val a: Boolean = nativeStruct.a > 0
    val b: UInt = nativeStruct.b.toUInt()

    companion object {
        internal val libClass: Class<StructWithAttrsLib> = StructWithAttrsLib::class.java
        internal val lib: StructWithAttrsLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(StructWithAttrsNative::class.java).toLong()
        @JvmStatic
        
        fun newFallible(a: Boolean, b: UInt): Result<StructWithAttrs> {
            
            val returnVal = lib.namespace_StructWithAttrs_new_fallible(a, FFIUint32(b));
            if (returnVal.isOk == 1.toByte()) {
                
                val returnStruct = StructWithAttrs(returnVal.union.ok)
                return returnStruct.ok()
            } else {
                return UnitError().err()
            }
        }
    }
    
    fun c(): UInt {
        
        val returnVal = lib.namespace_StructWithAttrs_c(nativeStruct);
        return (returnVal.toUInt())
    }
    
    fun deprecated(): Unit {
        
        val returnVal = lib.namespace_StructWithAttrs_deprecated(nativeStruct);
        
    }

}
