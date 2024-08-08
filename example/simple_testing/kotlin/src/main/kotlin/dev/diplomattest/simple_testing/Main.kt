package dev.diplomattest.simple_testing;

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

interface DiplomatCallback_Lib: Library {
    // specific to this callback type
    // this is the "run_callback" callback
    interface Runner_DiplomatCallback_Wrapper_test_rust_fn_callback_f: Callback {
        fun invoke(ignored: Pointer?, input: Int): Int
    }
    // specific to this callback
    fun Wrapper_test_multi_arg_callback(diplomatCallback: DiplomatCallback_Wrapper_test_rust_fn_callback_f_Native, i: Int): Int
}

class DiplomatCallback_Wrapper_test_rust_fn_callback_f_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var dataa: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: DiplomatCallback_Lib.Runner_DiplomatCallback_Wrapper_test_rust_fn_callback_f
        = object :  DiplomatCallback_Lib.Runner_DiplomatCallback_Wrapper_test_rust_fn_callback_f {
                override fun invoke(ignored: Pointer?, input: Int): Int {
                    throw Exception("ERROR NOT IMPLEMENTED")
                }
            }
    @JvmField
    internal var destructor: Pointer = Pointer(0L);
  
    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("dataa", "run_callback", "destructor")
    }
}

class DiplomatCallback_Wrapper_test_rust_fn_callback_f internal constructor (
    internal val nativeStruct: DiplomatCallback_Wrapper_test_rust_fn_callback_f_Native) {
    val dataa: Pointer = nativeStruct.dataa
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Pointer = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_Wrapper_test_rust_fn_callback_f_Native::class.java).toLong()
        internal val libClass: Class<DiplomatCallback_Lib> = DiplomatCallback_Lib::class.java
        internal val lib: DiplomatCallback_Lib = Native.load("somelib", libClass)

        fun fromCallback(cb: (Int) -> Int): DiplomatCallback_Wrapper_test_rust_fn_callback_f {
            val callback: DiplomatCallback_Lib.Runner_DiplomatCallback_Wrapper_test_rust_fn_callback_f = object :  DiplomatCallback_Lib.Runner_DiplomatCallback_Wrapper_test_rust_fn_callback_f {
                override fun invoke(ignored: Pointer?, input: Int): Int {
                    return cb(input);
                }
            }
            val cb_wrap = DiplomatCallback_Wrapper_test_rust_fn_callback_f_Native()
            cb_wrap.run_callback = callback;
            return DiplomatCallback_Wrapper_test_rust_fn_callback_f(cb_wrap)
        }

        fun call_test_rust_fn(cb_wrap: DiplomatCallback_Wrapper_test_rust_fn_callback_f, i: Int): Int {
            return lib.Wrapper_test_multi_arg_callback(cb_wrap.nativeStruct, i);
        }
    }

}

object Main {

    fun callback(x: Int): Int {
        return x*2;
    }
    
    @JvmStatic
    fun main(args: Array<String>) {
        var cb_wrap = DiplomatCallback_Wrapper_test_rust_fn_callback_f.fromCallback(::callback)
        var res = DiplomatCallback_Wrapper_test_rust_fn_callback_f.call_test_rust_fn(cb_wrap, 5); // calls test_rust_fn with the callback
        println("Result: " + res);
    }
}