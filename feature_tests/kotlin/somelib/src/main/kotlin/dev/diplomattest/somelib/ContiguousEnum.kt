package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ContiguousEnumLib: Library {
}
enum class ContiguousEnum {
    C,
    D,
    E,
    F;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ContiguousEnumLib> = ContiguousEnumLib::class.java
        internal val lib: ContiguousEnumLib = Native.load("somelib", libClass) 
        fun fromNative(native: Int): ContiguousEnum {
            return ContiguousEnum.entries[native]
        }

        fun default(): ContiguousEnum {
            return C
        }
    }
}