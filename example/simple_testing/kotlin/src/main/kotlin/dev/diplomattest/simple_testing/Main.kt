package dev.diplomattest.simple_testing

import com.sun.jna.JNIEnv
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer

import java.util.Collections
import java.util.concurrent.Callable

class CallbackWrapper(val cb: (Int) -> Int) {
    var arg0: Int = Int.MIN_VALUE

    fun set_arg0(new_arg0: Int) {
        arg0 = new_arg0
        println("here!! arg0 " + arg0)
    }

    fun run_callback(): Int {
        println("hereeeee")
        if (arg0 == Int.MIN_VALUE) {
            // throw an error
            // but also this sucks as error checking
        }
        val ret = this.cb.invoke(arg0);
        println("here: " + ret);
        return ret;
    }
}

interface DiplomatCallback_Lib: Library {
    // general use
    fun diplomat_callback_destroy(diplomatCallback: Pointer)
    fun diplomat_callback_create_for_jvm__callback(env: JNIEnv, callback_wrapper: Object): Pointer

    // specific to this callback
    fun GEND_BRIDGE_test_run_fn(diplomatCallback: Pointer): Int
}

class DiplomatCallback internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    // Note: ^ this comment was copied from other diplomat code
    internal val selfEdges: List<Any>
)  {

    internal class DiplomatCallbackCleaner(val handle: Pointer, val lib: DiplomatCallback_Lib) : Runnable {
        override fun run() {
            // TODO write this wrapper around the destructor
            // lib.DiplomatCallback_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<DiplomatCallback_Lib> = DiplomatCallback_Lib::class.java
        internal val lib: DiplomatCallback_Lib = Native.load("somelib", libClass,
            Collections.singletonMap(Library.OPTION_ALLOW_OBJECTS, true))

        fun diplomat_callback_create_for_jvm(callback_wrapper: CallbackWrapper): Pointer {
            return lib.diplomat_callback_create_for_jvm__callback(JNIEnv.CURRENT, callback_wrapper as Object);
        }

        fun GEND_BRIDGE_test_run_fn(diplomatCallback: Pointer): Int {
            return lib.GEND_BRIDGE_test_run_fn(diplomatCallback);
        }
    }
}

object Main {

    fun callback(x: Int): Int {
        return x*2;
    }

    fun callTestFunRust(cb: (Int) -> Int): Int {
        var cb_wrapper = CallbackWrapper(cb);
        var diplomat_cb_lib = DiplomatCallback.diplomat_callback_create_for_jvm(cb_wrapper);
        return DiplomatCallback.GEND_BRIDGE_test_run_fn(diplomat_cb_lib)
    }

    @JvmStatic
    fun main(args: Array<String>) {
        var res = callTestFunRust(::callback);
        println("Result: " + res);
    }
}