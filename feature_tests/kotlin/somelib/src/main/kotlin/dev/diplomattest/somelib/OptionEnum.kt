package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OptionEnumLib: Library {
}
enum class OptionEnum {
    Foo,
    Bar,
    Baz;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<OptionEnumLib> = OptionEnumLib::class.java
        internal val lib: OptionEnumLib = Native.load("diplomat_feature_tests", libClass) 
        fun fromNative(native: Int): OptionEnum {
            return OptionEnum.entries[native]
        }

        fun default(): OptionEnum {
            return Foo
        }
    }
}
