package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CyclicStructALib: Library {
    fun CyclicStructA_get_b(): CyclicStructBNative
    fun CyclicStructA_cyclic_out(nativeStruct: CyclicStructANative, write: Pointer): Unit
    fun CyclicStructA_double_cyclic_out(nativeStruct: CyclicStructANative, cyclicStructA: CyclicStructANative, write: Pointer): Unit
    fun CyclicStructA_getter_out(nativeStruct: CyclicStructANative, write: Pointer): Unit
}

internal class CyclicStructANative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: CyclicStructBNative = CyclicStructBNative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a")
    }
}




internal class OptionCyclicStructANative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: CyclicStructANative = CyclicStructANative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): CyclicStructANative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: CyclicStructANative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: CyclicStructANative): OptionCyclicStructANative {
            return OptionCyclicStructANative(value, 1)
        }

        internal fun none(): OptionCyclicStructANative {
            return OptionCyclicStructANative(CyclicStructANative(), 0)
        }
    }

}

class CyclicStructA (var a: CyclicStructB) {
    companion object {

        internal val libClass: Class<CyclicStructALib> = CyclicStructALib::class.java
        internal val lib: CyclicStructALib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CyclicStructANative::class.java).toLong()

        internal fun fromNative(nativeStruct: CyclicStructANative): CyclicStructA {
            val a: CyclicStructB = CyclicStructB.fromNative(nativeStruct.a)

            return CyclicStructA(a)
        }

        @JvmStatic
        
        fun getB(): CyclicStructB {
            
            val returnVal = lib.CyclicStructA_get_b();
            val returnStruct = CyclicStructB.fromNative(returnVal)
            return returnStruct
        }
    }
    internal fun toNative(): CyclicStructANative {
        var native = CyclicStructANative()
        native.a = this.a.toNative()
        return native
    }

    
    fun cyclicOut(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.CyclicStructA_cyclic_out(this.toNative(), write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    fun doubleCyclicOut(cyclicStructA: CyclicStructA): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.CyclicStructA_double_cyclic_out(this.toNative(), cyclicStructA.toNative(), write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    fun getterOut(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.CyclicStructA_getter_out(this.toNative(), write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
}