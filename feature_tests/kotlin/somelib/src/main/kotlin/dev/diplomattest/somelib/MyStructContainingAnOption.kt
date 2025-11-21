package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MyStructContainingAnOptionLib: Library {
    fun MyStructContainingAnOption_new(): MyStructContainingAnOptionNative
    fun MyStructContainingAnOption_filled(): MyStructContainingAnOptionNative
}

internal class MyStructContainingAnOptionNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: OptionMyStructNative = OptionMyStructNative.none();
    @JvmField
    internal var b: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a", "b")
    }
}




internal class OptionMyStructContainingAnOptionNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: MyStructContainingAnOptionNative = MyStructContainingAnOptionNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): MyStructContainingAnOptionNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: MyStructContainingAnOptionNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: MyStructContainingAnOptionNative): OptionMyStructContainingAnOptionNative {
            return OptionMyStructContainingAnOptionNative(value, 1)
        }

        internal fun none(): OptionMyStructContainingAnOptionNative {
            return OptionMyStructContainingAnOptionNative(MyStructContainingAnOptionNative(), 0)
        }
    }

}

class MyStructContainingAnOption internal constructor (
    internal val nativeStruct: MyStructContainingAnOptionNative) {
    val a: MyStruct? = nativeStruct.a.option()?.let { MyStruct(it) }
    val b: DefaultEnum? = nativeStruct.b.option()?.let { DefaultEnum.fromNative(it) }

    companion object {
        internal val libClass: Class<MyStructContainingAnOptionLib> = MyStructContainingAnOptionLib::class.java
        internal val lib: MyStructContainingAnOptionLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(MyStructContainingAnOptionNative::class.java).toLong()
        @JvmStatic
        
        fun new_(): MyStructContainingAnOption {
            
            val returnVal = lib.MyStructContainingAnOption_new();
            
            val returnStruct = MyStructContainingAnOption(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun filled(): MyStructContainingAnOption {
            
            val returnVal = lib.MyStructContainingAnOption_filled();
            
            val returnStruct = MyStructContainingAnOption(returnVal)
            return returnStruct
        }
    }

}
