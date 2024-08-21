package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface FixedDecimalGroupingStrategyLib: Library {
}
enum class FixedDecimalGroupingStrategy {
    Auto,
    Never,
    Always,
    Min2;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<FixedDecimalGroupingStrategyLib> = FixedDecimalGroupingStrategyLib::class.java
        internal val lib: FixedDecimalGroupingStrategyLib = Native.load("somelib", libClass) 
        fun fromNative(native: Int): FixedDecimalGroupingStrategy {
            return FixedDecimalGroupingStrategy.entries[native]
        }

        fun default(): FixedDecimalGroupingStrategy {
            return Auto
        }
    }
}