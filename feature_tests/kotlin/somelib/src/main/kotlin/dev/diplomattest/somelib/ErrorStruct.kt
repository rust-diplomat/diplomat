package dev.diplomattest.somelib

import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ErrorStructLib: Library {
}

class ErrorStructNative: Structure(), Structure.ByValue {
    @JvmField
    var i: Int = 0;
    @JvmField
    var j: Int = 0;
  
    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("i", "j")
    }
}

class ErrorStruct internal constructor (
    internal val nativeStruct: ErrorStructNative) {
    val i: Int = nativeStruct.i
    val j: Int = nativeStruct.j

    companion object {
        internal val libClass: Class<ErrorStructLib> = ErrorStructLib::class.java
        internal val lib: ErrorStructLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(ErrorStructNative::class.java).toLong()
    }

}
