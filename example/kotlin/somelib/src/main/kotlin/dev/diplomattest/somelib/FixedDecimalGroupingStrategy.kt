package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface FixedDecimalGroupingStrategyLib: Library {
    fun icu4x_FixedDecimalGroupingStrategy_groups_digits_mv1(inner: Int): Byte
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
        internal val lib: FixedDecimalGroupingStrategyLib = Native.load("diplomat_example", libClass) 
        fun fromNative(native: Int): FixedDecimalGroupingStrategy {
            return FixedDecimalGroupingStrategy.entries[native]
        }

        fun default(): FixedDecimalGroupingStrategy {
            return Auto
        }
    }
    
    /**
     * Whether this strategy can group digits at all.
     */
    fun groupsDigits(): Boolean {
        
        val returnVal = lib.icu4x_FixedDecimalGroupingStrategy_groups_digits_mv1(this.toNative());
        return (returnVal > 0)
    }
}
