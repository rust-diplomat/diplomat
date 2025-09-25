package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ErrorStructLib: Library {
}

internal class ErrorStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var i: Int = 0;
    @JvmField
    internal var j: Int = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("i", "j")
    }
}

class ErrorStruct internal constructor (
    internal val nativeStruct: ErrorStructNative): Exception("Rust error result for ErrorStruct") {
    val i: Int = nativeStruct.i
    val j: Int = nativeStruct.j

    companion object {
        internal val libClass: Class<ErrorStructLib> = ErrorStructLib::class.java
        internal val lib: ErrorStructLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(ErrorStructNative::class.java).toLong()
    }

}
