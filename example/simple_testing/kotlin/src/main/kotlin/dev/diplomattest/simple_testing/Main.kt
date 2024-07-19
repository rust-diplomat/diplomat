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

class DiplomatCallback internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    // Note: ^ this comment was copied from other diplomat code
    internal val selfEdges: List<Any>
)  {
    companion object {
        internal val libClass: Class<DiplomatCallback_Lib> = DiplomatCallback_Lib::class.java
        internal val lib: DiplomatCallback_Lib = Native.load("somelib", libClass)

        // create the JNA Callback object
        internal val callback: DiplomatCallback_Lib.DiplomatCallbackI32ToI32 = object : DiplomatCallback_Lib.DiplomatCallbackI32ToI32{
            override fun invoke(input: Int): Int {
                return input*2;
            }
        }

        internal val callback_pointer = lib.diplomat_callback_create_for_jvm__callback(callback);

        fun test_rust_fn_call_with_callback(): Int {
            return lib.DiplomatCallbackI32ToI32_test_rust_fn_test_call(callback_pointer);
        }
    }
}

object Main {
    
    @JvmStatic
    fun main(args: Array<String>) {
        var res = DiplomatCallback.test_rust_fn_call_with_callback(); // calls test_rust_fn with 10, and the callback
        println("Result: " + res);
    }
}