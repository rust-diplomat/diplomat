package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedAttrEnumLib: Library {
}
enum class RenamedAttrEnum {
    A,
    B,
    Renamed;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<RenamedAttrEnumLib> = RenamedAttrEnumLib::class.java
        internal val lib: RenamedAttrEnumLib = Native.load("diplomat_feature_tests", libClass) 
        fun fromNative(native: Int): RenamedAttrEnum {
            return RenamedAttrEnum.entries[native]
        }

        fun default(): RenamedAttrEnum {
            return A
        }
    }
}
