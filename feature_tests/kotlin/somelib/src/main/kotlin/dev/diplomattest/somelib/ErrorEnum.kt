package dev.diplomattest.somelib

import com.sun.jna.Library
import com.sun.jna.Native

internal interface ErrorEnumLib: Library {
}
enum class ErrorEnum {
    Foo,
    Bar;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ErrorEnumLib> = ErrorEnumLib::class.java
        internal val lib: ErrorEnumLib = Native.load("somelib", libClass) 
        fun fromNative(native: Int): ErrorEnum {
            return ErrorEnum.entries[native]
        }

        fun default(): ErrorEnum {
            return Foo
        }
    }
}