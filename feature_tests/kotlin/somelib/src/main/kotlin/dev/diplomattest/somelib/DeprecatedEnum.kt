package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DeprecatedEnumLib: Library {
}
enum class DeprecatedEnum {
    A;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DeprecatedEnumLib> = DeprecatedEnumLib::class.java
        internal val lib: DeprecatedEnumLib = Native.load("somelib", libClass) 
        fun fromNative(native: Int): DeprecatedEnum {
            return DeprecatedEnum.entries[native]
        }

        fun default(): DeprecatedEnum {
            return A
        }
    }
}