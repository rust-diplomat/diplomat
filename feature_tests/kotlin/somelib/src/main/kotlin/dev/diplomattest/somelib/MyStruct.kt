package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MyStructLib: Library {
    fun MyStruct_new(): MyStructNative
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

class MyStruct internal constructor (
    internal val nativeStruct: MyStructNative) {
    val a: UByte = nativeStruct.a.toUByte()
    val b: Boolean = nativeStruct.b > 0
    val c: UByte = nativeStruct.c.toUByte()
    val d: ULong = nativeStruct.d.toULong()
    val e: Int = nativeStruct.e
    val f: Int = nativeStruct.f
    val g: MyEnum = MyEnum.fromNative(nativeStruct.g)

    companion object {
        internal val libClass: Class<MyStructLib> = MyStructLib::class.java
        internal val lib: MyStructLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(MyStructNative::class.java).toLong()
        @JvmStatic
        
        fun new_(): MyStruct {
            
            val returnVal = lib.MyStruct_new();
            
            val returnStruct = MyStruct(returnVal)
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
    
    fun intoA(): UByte {
        
        val returnVal = lib.MyStruct_into_a(nativeStruct);
        return (returnVal.toUByte())
    }

}
