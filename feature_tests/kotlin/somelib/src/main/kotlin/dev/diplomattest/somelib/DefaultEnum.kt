package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DefaultEnumLib: Library {
    fun DefaultEnum_new(): Int
}
enum class DefaultEnum {
    A,
    B;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DefaultEnumLib> = DefaultEnumLib::class.java
        internal val lib: DefaultEnumLib = Native.load("somelib", libClass) 
        fun fromNative(native: Int): DefaultEnum {
            return DefaultEnum.entries[native]
        }

        fun default(): DefaultEnum {
            return A
        }
        @JvmStatic
        
        fun new_(): DefaultEnum {
            
            val returnVal = lib.DefaultEnum_new();
            return (DefaultEnum.fromNative(returnVal))
        }
    }
}