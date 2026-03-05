package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedTestMacroStructLib: Library {
    fun namespace_TestMacroStruct_test_func(): FFISizet
    fun namespace_TestMacroStruct_test_meta(): RenamedTestMacroStructNative
}

internal class RenamedTestMacroStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: FFISizet = FFISizet();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a")
    }
}




internal class OptionRenamedTestMacroStructNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: RenamedTestMacroStructNative = RenamedTestMacroStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): RenamedTestMacroStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: RenamedTestMacroStructNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: RenamedTestMacroStructNative): OptionRenamedTestMacroStructNative {
            return OptionRenamedTestMacroStructNative(value, 1)
        }

        internal fun none(): OptionRenamedTestMacroStructNative {
            return OptionRenamedTestMacroStructNative(RenamedTestMacroStructNative(), 0)
        }
    }

}

class RenamedTestMacroStruct (var a: ULong) {
    companion object {

        internal val libClass: Class<RenamedTestMacroStructLib> = RenamedTestMacroStructLib::class.java
        internal val lib: RenamedTestMacroStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(RenamedTestMacroStructNative::class.java).toLong()

        internal fun fromNative(nativeStruct: RenamedTestMacroStructNative): RenamedTestMacroStruct {
            val a: ULong = nativeStruct.a.toULong()

            return RenamedTestMacroStruct(a)
        }

        @JvmStatic
        
        fun testFunc(): ULong {
            
            val returnVal = lib.namespace_TestMacroStruct_test_func();
            return (returnVal.toULong())
        }
        @JvmStatic
        
        fun testMeta(): RenamedTestMacroStruct {
            
            val returnVal = lib.namespace_TestMacroStruct_test_meta();
            val returnStruct = RenamedTestMacroStruct.fromNative(returnVal)
            return returnStruct
        }
    }
    internal fun toNative(): RenamedTestMacroStructNative {
        var native = RenamedTestMacroStructNative()
        native.a = FFISizet(this.a)
        return native
    }

}