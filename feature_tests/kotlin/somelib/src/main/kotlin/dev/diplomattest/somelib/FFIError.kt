package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface FFIErrorLib: Library {
}
enum class FFIError {
    FFI,
    User;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<FFIErrorLib> = FFIErrorLib::class.java
        internal val lib: FFIErrorLib = Native.load("diplomat_feature_tests", libClass) 
        fun fromNative(native: Int): FFIError {
            return FFIError.entries[native]
        }

        fun default(): FFIError {
            return FFI
        }
    }
}
