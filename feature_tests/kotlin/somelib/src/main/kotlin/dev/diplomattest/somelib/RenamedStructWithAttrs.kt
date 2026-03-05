package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedStructWithAttrsLib: Library {
    fun namespace_StructWithAttrs_new_fallible(a: Boolean, b: FFIUint32): ResultRenamedStructWithAttrsNativeUnit
    fun namespace_StructWithAttrs_c(nativeStruct: RenamedStructWithAttrsNative): FFIUint32
    fun namespace_StructWithAttrs_deprecated(nativeStruct: RenamedStructWithAttrsNative): Unit
}

internal class RenamedStructWithAttrsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: Byte = 0;
    @JvmField
    internal var b: FFIUint32 = FFIUint32();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a", "b")
    }
}




internal class OptionRenamedStructWithAttrsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: RenamedStructWithAttrsNative = RenamedStructWithAttrsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): RenamedStructWithAttrsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: RenamedStructWithAttrsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: RenamedStructWithAttrsNative): OptionRenamedStructWithAttrsNative {
            return OptionRenamedStructWithAttrsNative(value, 1)
        }

        internal fun none(): OptionRenamedStructWithAttrsNative {
            return OptionRenamedStructWithAttrsNative(RenamedStructWithAttrsNative(), 0)
        }
    }

}

class RenamedStructWithAttrs (var a: Boolean, var b: UInt) {
    companion object {

        internal val libClass: Class<RenamedStructWithAttrsLib> = RenamedStructWithAttrsLib::class.java
        internal val lib: RenamedStructWithAttrsLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(RenamedStructWithAttrsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: RenamedStructWithAttrsNative): RenamedStructWithAttrs {
            val a: Boolean = nativeStruct.a > 0
            val b: UInt = nativeStruct.b.toUInt()

            return RenamedStructWithAttrs(a, b)
        }

        @JvmStatic
        
        fun newFallible(a: Boolean, b: UInt): Result<RenamedStructWithAttrs> {
            
            val returnVal = lib.namespace_StructWithAttrs_new_fallible(a, FFIUint32(b));
            if (returnVal.isOk == 1.toByte()) {
                val returnStruct = RenamedStructWithAttrs.fromNative(returnVal.union.ok)
                return returnStruct.ok()
            } else {
                return UnitError().err()
            }
        }
    }
    internal fun toNative(): RenamedStructWithAttrsNative {
        var native = RenamedStructWithAttrsNative()
        native.a = if (this.a) 1 else 0
        native.b = FFIUint32(this.b)
        return native
    }

    
    fun c(): UInt {
        
        val returnVal = lib.namespace_StructWithAttrs_c(this.toNative());
        return (returnVal.toUInt())
    }
    
    fun deprecated(): Unit {
        
        val returnVal = lib.namespace_StructWithAttrs_deprecated(this.toNative());
        
    }
}