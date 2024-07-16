package dev.diplomattest.simple_testing

import com.sun.jna.JNIEnv
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer

import java.util.Collections;

internal interface FFIArgsWrapper_f_Lib: Library {
    fun FFIArgsWrapper_f_destroy(handle: Pointer)
    fun get_arg0_from_ffiargswrapper_f_pointer(handle: Pointer): Int
}

class FFIArgsWrapper_f internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
)  {

    internal class FFIArgsWrapper_fCleaner(val handle: Pointer, val lib: FFIArgsWrapper_f_Lib) : Runnable {
        override fun run() {
            lib.FFIArgsWrapper_f_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<FFIArgsWrapper_f_Lib> = FFIArgsWrapper_f_Lib::class.java
        internal val lib: FFIArgsWrapper_f_Lib = Native.load("somelib", libClass, 
            Collections.singletonMap(Library.OPTION_ALLOW_OBJECTS, true))

        fun get_arg0_from_ffiargswrapper_f_pointer(handle: Pointer): Int {
            return lib.get_arg0_from_ffiargswrapper_f_pointer(handle);
        }
    }
}

class CallbackWrapper(val cb: (Int) -> Int): Runnable {
    var args: Pointer = Pointer(0)

    fun set_args_pointer(native_arg_pointer: Long) {
        args = Pointer(native_arg_pointer);
    }

    override fun run() {
        if (Pointer.nativeValue(args) == 0L) {
            throw IllegalStateException("callback args not initialized (null pointer)");
        }

        var arg0 = FFIArgsWrapper_f.get_arg0_from_ffiargswrapper_f_pointer(args);

        val ret = this.cb.invoke(arg0);
        println("here: " + ret);
    }
}

interface DiplomatCallback_Lib: Library {
    // general use
    fun diplomat_callback_destroy(diplomatCallback: Pointer)
    fun diplomat_callback_create_for_jvm(env: JNIEnv, callback_wrapper: Object): Pointer

    // specific to this callback
    fun GEND_BRIDGE_test_run_fn(diplomatCallback: Pointer)
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

        fun diplomat_callback_create_for_jvm(callback_wrapper: Runnable): Pointer {
            return lib.diplomat_callback_create_for_jvm(JNIEnv.CURRENT, callback_wrapper as Object);
        }

        fun GEND_BRIDGE_test_run_fn(diplomatCallback: Pointer) {
            lib.GEND_BRIDGE_test_run_fn(diplomatCallback);
        }
    }
}

object Main {

    fun callback(x: Int): Int {
        return x*2;
    }

    fun callTestFunRust() {
        var cb_wrapper = CallbackWrapper(::callback);
        var diplomat_cb_lib = DiplomatCallback.diplomat_callback_create_for_jvm(cb_wrapper);
        DiplomatCallback.GEND_BRIDGE_test_run_fn(diplomat_cb_lib);
    }

    @JvmStatic
    fun main(args: Array<String>) {
        callTestFunRust();
    }
}