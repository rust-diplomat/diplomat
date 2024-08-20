package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CyclicStructALib: Library {
    fun CyclicStructA_get_b(): CyclicStructBNative
}

internal class CyclicStructANative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: CyclicStructBNative = CyclicStructBNative();
  
    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a")
    }
}

class CyclicStructA internal constructor (
    internal val nativeStruct: CyclicStructANative) {
    val a: CyclicStructB = CyclicStructB(nativeStruct.a)

    companion object {
        internal val libClass: Class<CyclicStructALib> = CyclicStructALib::class.java
        internal val lib: CyclicStructALib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CyclicStructANative::class.java).toLong()
        
        fun getB(): CyclicStructB {
            
            val returnVal = lib.CyclicStructA_get_b();
            
            val returnStruct = CyclicStructB(returnVal)
            return returnStruct
        }
    }

}
