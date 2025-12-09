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

class MyStructContainingAnOption (var a: MyStruct?, var b: DefaultEnum?) {
    companion object {

        internal val libClass: Class<MyStructContainingAnOptionLib> = MyStructContainingAnOptionLib::class.java
        internal val lib: MyStructContainingAnOptionLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(MyStructContainingAnOptionNative::class.java).toLong()

        internal fun fromNative(nativeStruct: MyStructContainingAnOptionNative): MyStructContainingAnOption {
            val a: MyStruct? = nativeStruct.a.option()?.let { MyStruct.fromNative(it) }
            val b: DefaultEnum? = nativeStruct.b.option()?.let { DefaultEnum.fromNative(it) }

            return MyStructContainingAnOption(a, b)
        }

        @JvmStatic
        
        fun new_(): MyStructContainingAnOption {
            
            val returnVal = lib.MyStructContainingAnOption_new();
            
            val returnStruct = MyStructContainingAnOption.fromNative(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun filled(): MyStructContainingAnOption {
            
            val returnVal = lib.MyStructContainingAnOption_filled();
            
            val returnStruct = MyStructContainingAnOption.fromNative(returnVal)
            return returnStruct
        }
    }
    internal fun toNative(): MyStructContainingAnOptionNative {
        var native = MyStructContainingAnOptionNative()
        native.a = this.a?.let { OptionMyStructNative.some(it.toNative()) } ?: OptionMyStructNative.none()
        native.b = this.b?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

}