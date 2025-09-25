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

class CyclicStructC internal constructor (
    internal val nativeStruct: CyclicStructCNative) {
    val a: CyclicStructA = CyclicStructA(nativeStruct.a)

    companion object {
        internal val libClass: Class<CyclicStructCLib> = CyclicStructCLib::class.java
        internal val lib: CyclicStructCLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CyclicStructCNative::class.java).toLong()
        @JvmStatic
        
        fun takesNestedParameters(c: CyclicStructC): CyclicStructC {
            
            val returnVal = lib.CyclicStructC_takes_nested_parameters(c.nativeStruct);
            
            val returnStruct = CyclicStructC(returnVal)
            return returnStruct
        }
    }
    
    fun cyclicOut(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.CyclicStructC_cyclic_out(nativeStruct, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}
