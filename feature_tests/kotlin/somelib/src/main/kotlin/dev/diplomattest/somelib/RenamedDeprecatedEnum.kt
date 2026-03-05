package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedDeprecatedEnumLib: Library {
}
enum class RenamedDeprecatedEnum {
    A;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<RenamedDeprecatedEnumLib> = RenamedDeprecatedEnumLib::class.java
        internal val lib: RenamedDeprecatedEnumLib = Native.load("diplomat_feature_tests", libClass) 
        fun fromNative(native: Int): RenamedDeprecatedEnum {
            return RenamedDeprecatedEnum.entries[native]
        }

        fun default(): RenamedDeprecatedEnum {
            return A
        }
    }
}
