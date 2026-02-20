package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MyStructLib: Library {
    fun MyStruct_new(): MyStructNative
    fun MyStruct_new_overload(i: Int): MyStructNative
    fun MyStruct_into_a(nativeStruct: MyStructNative): FFIUint8
    fun MyStruct_returns_zst_result(): ResultUnitMyZstNative
    fun MyStruct_fails_zst_result(): ResultUnitMyZstNative
}

internal class MyStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: FFIUint8 = FFIUint8();
    @JvmField
    internal var b: Byte = 0;
    @JvmField
    internal var c: FFIUint8 = FFIUint8();
    @JvmField
    internal var d: FFIUint64 = FFIUint64();
    @JvmField
    internal var e: Int = 0;
    @JvmField
    internal var f: Int = 0;
    @JvmField
    internal var g: Int = MyEnum.default().toNative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a", "b", "c", "d", "e", "f", "g")
    }
}




internal class OptionMyStructNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: MyStructNative = MyStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): MyStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: MyStructNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: MyStructNative): OptionMyStructNative {
            return OptionMyStructNative(value, 1)
        }

        internal fun none(): OptionMyStructNative {
            return OptionMyStructNative(MyStructNative(), 0)
        }
    }

}

class MyStruct (var a: UByte, var b: Boolean, var c: UByte, var d: ULong, var e: Int, var f: Int, var g: MyEnum) {
    companion object {

        internal val libClass: Class<MyStructLib> = MyStructLib::class.java
        internal val lib: MyStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(MyStructNative::class.java).toLong()

        internal fun fromNative(nativeStruct: MyStructNative): MyStruct {
            val a: UByte = nativeStruct.a.toUByte()
            val b: Boolean = nativeStruct.b > 0
            val c: UByte = nativeStruct.c.toUByte()
            val d: ULong = nativeStruct.d.toULong()
            val e: Int = nativeStruct.e
            val f: Int = nativeStruct.f
            val g: MyEnum = MyEnum.fromNative(nativeStruct.g)

            return MyStruct(a, b, c, d, e, f, g)
        }

        @JvmStatic
        
        fun new_(): MyStruct {
            
            val returnVal = lib.MyStruct_new();
            val returnStruct = MyStruct.fromNative(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun newOverload(i: Int): MyStruct {
            
            val returnVal = lib.MyStruct_new_overload(i);
            val returnStruct = MyStruct.fromNative(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun returnsZstResult(): Result<Unit> {
            
            val returnVal = lib.MyStruct_returns_zst_result();
            if (returnVal.isOk == 1.toByte()) {
                return Unit.ok()
            } else {
                return MyZst().err()
            }
        }
        @JvmStatic
        
        fun failsZstResult(): Result<Unit> {
            
            val returnVal = lib.MyStruct_fails_zst_result();
            if (returnVal.isOk == 1.toByte()) {
                return Unit.ok()
            } else {
                return MyZst().err()
            }
        }
    }
    internal fun toNative(): MyStructNative {
        var native = MyStructNative()
        native.a = FFIUint8(this.a)
        native.b = if (this.b) 1 else 0
        native.c = FFIUint8(this.c)
        native.d = FFIUint64(this.d)
        native.e = this.e
        native.f = this.f
        native.g = this.g.toNative()
        return native
    }

    
    fun intoA(): UByte {
        
        val returnVal = lib.MyStruct_into_a(this.toNative());
        return (returnVal.toUByte())
    }
}