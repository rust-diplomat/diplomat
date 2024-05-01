package dev.gigapixel.somelib

import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OptionStructLib: Library {
}

class OptionStructNative: Structure(), Structure.ByValue {
    @JvmField
    var a: Pointer? = null;
    @JvmField
    var b: Pointer? = null;
    @JvmField
    var c: Int = 0;
    @JvmField
    var d: Pointer? = null;
  
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
    val d: OptionOpaque? = if (nativeStruct.d == null) {
        null
    } else {
        OptionOpaque(nativeStruct.d!!, listOf())
    }

    companion object {
        internal val libClass: Class<OptionStructLib> = OptionStructLib::class.java
        internal val lib: OptionStructLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(OptionStructNative::class.java).toLong()
    }

}
