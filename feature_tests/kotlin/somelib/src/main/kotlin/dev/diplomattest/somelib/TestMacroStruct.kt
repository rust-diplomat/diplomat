package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TestMacroStructLib: Library {
    fun namespace_TestMacroStruct_test_func(): FFISizet
    fun namespace_TestMacroStruct_test_meta(): TestMacroStructNative
}

internal class TestMacroStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: FFISizet = FFISizet();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a")
    }
}




internal class OptionTestMacroStructNative: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: TestMacroStructNative = TestMacroStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): TestMacroStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}

class TestMacroStruct internal constructor (
    internal val nativeStruct: TestMacroStructNative) {
    val a: ULong = nativeStruct.a.toULong()

    companion object {
        internal val libClass: Class<TestMacroStructLib> = TestMacroStructLib::class.java
        internal val lib: TestMacroStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(TestMacroStructNative::class.java).toLong()
        @JvmStatic
        
        fun testFunc(): ULong {
            
            val returnVal = lib.namespace_TestMacroStruct_test_func();
            return (returnVal.toULong())
        }
        @JvmStatic
        
        fun testMeta(): TestMacroStruct {
            
            val returnVal = lib.namespace_TestMacroStruct_test_meta();
            
            val returnStruct = TestMacroStruct(returnVal)
            return returnStruct
        }
    }

}
