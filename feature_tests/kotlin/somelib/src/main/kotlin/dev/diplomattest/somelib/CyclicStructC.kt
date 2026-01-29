package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CyclicStructCLib: Library {
    fun CyclicStructC_takes_nested_parameters(c: CyclicStructCNative): CyclicStructCNative
    fun CyclicStructC_cyclic_out(nativeStruct: CyclicStructCNative, write: Pointer): Unit
}

internal class CyclicStructCNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: CyclicStructANative = CyclicStructANative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a")
    }
}




internal class OptionCyclicStructCNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: CyclicStructCNative = CyclicStructCNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): CyclicStructCNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: CyclicStructCNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: CyclicStructCNative): OptionCyclicStructCNative {
            return OptionCyclicStructCNative(value, 1)
        }

        internal fun none(): OptionCyclicStructCNative {
            return OptionCyclicStructCNative(CyclicStructCNative(), 0)
        }
    }

}

class CyclicStructC (var a: CyclicStructA) {
    companion object {

        internal val libClass: Class<CyclicStructCLib> = CyclicStructCLib::class.java
        internal val lib: CyclicStructCLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CyclicStructCNative::class.java).toLong()

        internal fun fromNative(nativeStruct: CyclicStructCNative): CyclicStructC {
            val a: CyclicStructA = CyclicStructA.fromNative(nativeStruct.a)

            return CyclicStructC(a)
        }

        @JvmStatic
        
        fun takesNestedParameters(c: CyclicStructC): CyclicStructC {
            
            val returnVal = lib.CyclicStructC_takes_nested_parameters(c.toNative());
            val returnStruct = CyclicStructC.fromNative(returnVal)
            return returnStruct
        }
    }
    internal fun toNative(): CyclicStructCNative {
        var native = CyclicStructCNative()
        native.a = this.a.toNative()
        return native
    }

    
    fun cyclicOut(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.CyclicStructC_cyclic_out(this.toNative(), write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
}