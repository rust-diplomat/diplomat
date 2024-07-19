package dev.diplomattest.simple_testing;

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


interface DiplomatCallback_Lib: Library {
    // general use
    fun diplomat_callback_create_for_jvm__callback(callback_wrap: Callback): Pointer

    // specific to this callback type
    interface DiplomatCallbackI32ToI32: Callback {
        fun invoke(input: Int): Int
    }
    // specific to this callback
    fun DiplomatCallbackI32ToI32_test_rust_fn_test_call(diplomatCallback: Pointer): Int
}

class DiplomatCallbackI32ToI32(
    val callback_pointer: Pointer,
    val callback: DiplomatCallback_Lib.DiplomatCallbackI32ToI32,
)  {

    companion object {
        internal val libClass: Class<DiplomatCallback_Lib> = DiplomatCallback_Lib::class.java
        internal val lib: DiplomatCallback_Lib = Native.load("somelib", libClass)

        fun test_rust_fn_call_with_callback(dc: DiplomatCallbackI32ToI32): Int {
            return lib.DiplomatCallbackI32ToI32_test_rust_fn_test_call(dc.callback_pointer);
        }

        fun fromCallback(cb: (Int) -> Int): DiplomatCallbackI32ToI32 {
            val callback: DiplomatCallback_Lib.DiplomatCallbackI32ToI32 = object :  DiplomatCallback_Lib.DiplomatCallbackI32ToI32 {
                override fun invoke(input: Int): Int {
                    return cb(input);
                }
            }
            val callback_pointer = lib.diplomat_callback_create_for_jvm__callback(callback);
            return DiplomatCallbackI32ToI32(callback_pointer, callback)
        }
    }
}

object Main {

    fun callback(x: Int): Int {
        return x*2;
    }
    
    @JvmStatic
    fun main(args: Array<String>) {
        var cb_wrap = DiplomatCallbackI32ToI32.fromCallback(::callback)
        var res = DiplomatCallbackI32ToI32.test_rust_fn_call_with_callback(cb_wrap); // calls test_rust_fn with the callback
        println("Result: " + res);
    }
}