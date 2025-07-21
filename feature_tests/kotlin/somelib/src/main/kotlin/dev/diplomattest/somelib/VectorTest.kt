package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface VectorTestLib: Library {
    fun namespace_VectorTest_new(): VectorTestNative
}

internal class VectorTestNative: Structure(), Structure.ByValue {
    @JvmField
    internal var test: Double = 0.0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("test")
    }
}

class VectorTest internal constructor (
    internal val nativeStruct: VectorTestNative) {
    val test: Double = nativeStruct.test

    companion object {
        internal val libClass: Class<VectorTestLib> = VectorTestLib::class.java
        internal val lib: VectorTestLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(VectorTestNative::class.java).toLong()
        @JvmStatic
        
        fun new_(): VectorTest {
            
            val returnVal = lib.namespace_VectorTest_new();
            
            val returnStruct = VectorTest(returnVal)
            return returnStruct
        }
    }

}
