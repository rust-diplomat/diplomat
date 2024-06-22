package dev.diplomattest.somelib

import com.sun.jna.Library
import com.sun.jna.Native

internal interface ICU4XFixedDecimalGroupingStrategyLib: Library {
}
enum class ICU4XFixedDecimalGroupingStrategy {
    Auto,
    Never,
    Always,
    Min2;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ICU4XFixedDecimalGroupingStrategyLib> = ICU4XFixedDecimalGroupingStrategyLib::class.java
        internal val lib: ICU4XFixedDecimalGroupingStrategyLib = Native.load("somelib", libClass) 
        fun fromNative(native: Int): ICU4XFixedDecimalGroupingStrategy {
            return ICU4XFixedDecimalGroupingStrategy.entries[native]
        }

        fun default(): ICU4XFixedDecimalGroupingStrategy {
            return Auto
        }
    }
}