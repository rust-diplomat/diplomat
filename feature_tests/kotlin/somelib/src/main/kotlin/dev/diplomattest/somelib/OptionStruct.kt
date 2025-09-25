package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OptionStructLib: Library {
}

internal class OptionStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var a: Pointer? = null;
    @JvmField
    internal var b: Pointer? = null;
    @JvmField
    internal var c: FFIUint32 = FFIUint32();
    @JvmField
    internal var d: Pointer = Pointer(0);

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("a", "b", "c", "d")
    }
}

class OptionStruct internal constructor (
    internal val nativeStruct: OptionStructNative) {
    val a: OptionOpaque? = if (nativeStruct.a == null) {
        null
    } else {
        OptionOpaque(nativeStruct.a!!, listOf())
    }
    val b: OptionOpaqueChar? = if (nativeStruct.b == null) {
        null
    } else {
        OptionOpaqueChar(nativeStruct.b!!, listOf())
    }
    val c: UInt = nativeStruct.c.toUInt()
    val d: OptionOpaque = OptionOpaque(nativeStruct.d, listOf())

    companion object {
        internal val libClass: Class<OptionStructLib> = OptionStructLib::class.java
        internal val lib: OptionStructLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(OptionStructNative::class.java).toLong()
    }

}
