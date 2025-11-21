package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface UnimportedEnumLib: Library {
}
enum class UnimportedEnum {
    A,
    B,
    C;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<UnimportedEnumLib> = UnimportedEnumLib::class.java
        internal val lib: UnimportedEnumLib = Native.load("diplomat_feature_tests", libClass) 
        fun fromNative(native: Int): UnimportedEnum {
            return UnimportedEnum.entries[native]
        }

        fun default(): UnimportedEnum {
            return A
        }
    }
}
