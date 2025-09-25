package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MyEnumLib: Library {
    fun MyEnum_into_value(inner: Int): Byte
    fun MyEnum_get_a(): Int
}
enum class MyEnum(val inner: Int) {
    A(-2),
    B(-1),
    C(0),
    D(1),
    E(2),
    F(3);

    fun toNative(): Int {
        return this.inner
    }


    companion object {
        internal val libClass: Class<MyEnumLib> = MyEnumLib::class.java
        internal val lib: MyEnumLib = Native.load("somelib", libClass)
        fun fromNative(native: Int): MyEnum {
            return when (native) {
                -2 -> A
                -1 -> B
                0 -> C
                1 -> D
                2 -> E
                3 -> F
                else -> throw RuntimeException("Failed to find variant ${native} of type MyEnum")
            }
        }

        fun default(): MyEnum {
            return A
        }
        @JvmStatic
        
        fun getA(): MyEnum {
            
            val returnVal = lib.MyEnum_get_a();
            return (MyEnum.fromNative(returnVal))
        }
    }
    
    fun intoValue(): Byte {
        
        val returnVal = lib.MyEnum_into_value(this.toNative());
        return (returnVal)
    }
}