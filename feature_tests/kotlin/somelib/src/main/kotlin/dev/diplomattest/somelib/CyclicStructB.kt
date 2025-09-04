package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CyclicStructBLib: Library {
    fun CyclicStructB_get_a(): CyclicStructANative
    fun CyclicStructB_get_a_option(): OptionCyclicStructANative
}

internal class CyclicStructBNative: Structure(), Structure.ByValue {
    @JvmField
    internal var field: FFIUint8 = FFIUint8();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("field")
    }
}

class CyclicStructB internal constructor (
    internal val nativeStruct: CyclicStructBNative) {
    val field: UByte = nativeStruct.field.toUByte()

    companion object {
        internal val libClass: Class<CyclicStructBLib> = CyclicStructBLib::class.java
        internal val lib: CyclicStructBLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CyclicStructBNative::class.java).toLong()
        @JvmStatic
        
        fun getA(): CyclicStructA {
            
            val returnVal = lib.CyclicStructB_get_a();
            
            val returnStruct = CyclicStructA(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun getAOption(): CyclicStructA? {
            
            val returnVal = lib.CyclicStructB_get_a_option();
            
            val intermediateOption = returnVal.option() ?: return null

            val returnStruct = CyclicStructA(intermediateOption)
            return returnStruct
                                    
        }
    }

}
