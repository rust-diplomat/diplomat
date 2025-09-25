package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface AttrEnumLib: Library {
}
enum class AttrEnum {
    A,
    B,
    C;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<AttrEnumLib> = AttrEnumLib::class.java
        internal val lib: AttrEnumLib = Native.load("somelib", libClass) 
        fun fromNative(native: Int): AttrEnum {
            return AttrEnum.entries[native]
        }

        fun default(): AttrEnum {
            return A
        }
    }
}