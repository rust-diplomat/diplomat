package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CallbackTestingStructLib: Library {
}

internal class CallbackTestingStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var x: Int = 0;
    @JvmField
    internal var y: Int = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("x", "y")
    }
}

class CallbackTestingStruct internal constructor (
    internal val nativeStruct: CallbackTestingStructNative) {
    val x: Int = nativeStruct.x
    val y: Int = nativeStruct.y

    companion object {
        internal val libClass: Class<CallbackTestingStructLib> = CallbackTestingStructLib::class.java
        internal val lib: CallbackTestingStructLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CallbackTestingStructNative::class.java).toLong()
    }

}
