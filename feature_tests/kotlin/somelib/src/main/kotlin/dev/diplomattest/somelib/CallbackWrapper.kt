package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CallbackWrapperLib: Library {
    fun CallbackWrapper_test_multi_arg_callback(f: DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f_Native, x: Int): Int
    fun CallbackWrapper_test_no_args(h: DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h_Native): Int
    fun CallbackWrapper_test_cb_with_struct(f: DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f_Native): Int
    fun CallbackWrapper_test_multiple_cb_args(f: DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f_Native, g: DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g_Native): Int
    fun CallbackWrapper_test_slice_cb_arg(arg: Slice, f: DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f_Native): Unit
}

internal class CallbackWrapperNative: Structure(), Structure.ByValue {
    @JvmField
    internal var cantBeEmpty: Byte = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("cantBeEmpty")
    }
}


internal interface Runner_DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f: Callback {
    fun invoke(lang_specific_context: Pointer?, arg0: Int ): Int
}

internal class DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var data_: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f {
                override fun invoke(lang_specific_context: Pointer?, arg0: Int ): Int {
                    throw Exception("Default callback runner -- should be replaced.")
                }
            }
    @JvmField
    internal var destructor: Callback = object : Callback {
        fun invoke(obj_pointer: Pointer) {
            DiplomatJVMRuntime.dropRustCookie(obj_pointer);
        }
    };

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("data_", "run_callback", "destructor")
    }
}

internal class DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f_Native) {
    val data_: Pointer = nativeStruct.data_
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Callback = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f_Native::class.java).toLong()

        fun fromCallback(cb: (Int)->Int): DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f = object :  Runner_DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f {
                override fun invoke(lang_specific_context: Pointer?, arg0: Int ): Int {
                    return cb(arg0);
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f_Native()
            cb_wrap.run_callback = callback;
            cb_wrap.data_ = DiplomatJVMRuntime.buildRustCookie(cb_wrap as Object);
            return DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f(cb_wrap)
        }
    }
}
internal interface Runner_DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h: Callback {
    fun invoke(lang_specific_context: Pointer?): Unit
}

internal class DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var data_: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h {
                override fun invoke(lang_specific_context: Pointer?): Unit {
                    throw Exception("Default callback runner -- should be replaced.")
                }
            }
    @JvmField
    internal var destructor: Callback = object : Callback {
        fun invoke(obj_pointer: Pointer) {
            DiplomatJVMRuntime.dropRustCookie(obj_pointer);
        }
    };

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("data_", "run_callback", "destructor")
    }
}

internal class DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h_Native) {
    val data_: Pointer = nativeStruct.data_
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Callback = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h_Native::class.java).toLong()

        fun fromCallback(cb: ()->Unit): DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h = object :  Runner_DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h {
                override fun invoke(lang_specific_context: Pointer?): Unit {
                    return cb();
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h_Native()
            cb_wrap.run_callback = callback;
            cb_wrap.data_ = DiplomatJVMRuntime.buildRustCookie(cb_wrap as Object);
            return DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h(cb_wrap)
        }
    }
}
internal interface Runner_DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f: Callback {
    fun invoke(lang_specific_context: Pointer?, arg0: CallbackTestingStructNative ): Int
}

internal class DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var data_: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f {
                override fun invoke(lang_specific_context: Pointer?, arg0: CallbackTestingStructNative ): Int {
                    throw Exception("Default callback runner -- should be replaced.")
                }
            }
    @JvmField
    internal var destructor: Callback = object : Callback {
        fun invoke(obj_pointer: Pointer) {
            DiplomatJVMRuntime.dropRustCookie(obj_pointer);
        }
    };

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("data_", "run_callback", "destructor")
    }
}

internal class DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f_Native) {
    val data_: Pointer = nativeStruct.data_
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Callback = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f_Native::class.java).toLong()

        fun fromCallback(cb: (CallbackTestingStruct)->Int): DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f = object :  Runner_DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f {
                override fun invoke(lang_specific_context: Pointer?, arg0: CallbackTestingStructNative ): Int {
                    return cb(CallbackTestingStruct(arg0));
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f_Native()
            cb_wrap.run_callback = callback;
            cb_wrap.data_ = DiplomatJVMRuntime.buildRustCookie(cb_wrap as Object);
            return DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f(cb_wrap)
        }
    }
}
internal interface Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f: Callback {
    fun invoke(lang_specific_context: Pointer?): Int
}

internal class DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var data_: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f {
                override fun invoke(lang_specific_context: Pointer?): Int {
                    throw Exception("Default callback runner -- should be replaced.")
                }
            }
    @JvmField
    internal var destructor: Callback = object : Callback {
        fun invoke(obj_pointer: Pointer) {
            DiplomatJVMRuntime.dropRustCookie(obj_pointer);
        }
    };

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("data_", "run_callback", "destructor")
    }
}

internal class DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f_Native) {
    val data_: Pointer = nativeStruct.data_
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Callback = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f_Native::class.java).toLong()

        fun fromCallback(cb: ()->Int): DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f = object :  Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f {
                override fun invoke(lang_specific_context: Pointer?): Int {
                    return cb();
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f_Native()
            cb_wrap.run_callback = callback;
            cb_wrap.data_ = DiplomatJVMRuntime.buildRustCookie(cb_wrap as Object);
            return DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f(cb_wrap)
        }
    }
}
internal interface Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g: Callback {
    fun invoke(lang_specific_context: Pointer?, arg0: Int ): Int
}

internal class DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var data_: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g {
                override fun invoke(lang_specific_context: Pointer?, arg0: Int ): Int {
                    throw Exception("Default callback runner -- should be replaced.")
                }
            }
    @JvmField
    internal var destructor: Callback = object : Callback {
        fun invoke(obj_pointer: Pointer) {
            DiplomatJVMRuntime.dropRustCookie(obj_pointer);
        }
    };

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("data_", "run_callback", "destructor")
    }
}

internal class DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g_Native) {
    val data_: Pointer = nativeStruct.data_
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Callback = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g_Native::class.java).toLong()

        fun fromCallback(cb: (Int)->Int): DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g = object :  Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g {
                override fun invoke(lang_specific_context: Pointer?, arg0: Int ): Int {
                    return cb(arg0);
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g_Native()
            cb_wrap.run_callback = callback;
            cb_wrap.data_ = DiplomatJVMRuntime.buildRustCookie(cb_wrap as Object);
            return DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g(cb_wrap)
        }
    }
}
internal interface Runner_DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f: Callback {
    fun invoke(lang_specific_context: Pointer?, arg0: Slice ): Unit
}

internal class DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var data_: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f {
                override fun invoke(lang_specific_context: Pointer?, arg0: Slice ): Unit {
                    throw Exception("Default callback runner -- should be replaced.")
                }
            }
    @JvmField
    internal var destructor: Callback = object : Callback {
        fun invoke(obj_pointer: Pointer) {
            DiplomatJVMRuntime.dropRustCookie(obj_pointer);
        }
    };

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("data_", "run_callback", "destructor")
    }
}

internal class DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f_Native) {
    val data_: Pointer = nativeStruct.data_
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Callback = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f_Native::class.java).toLong()

        fun fromCallback(cb: (UByteArray)->Unit): DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f = object :  Runner_DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f {
                override fun invoke(lang_specific_context: Pointer?, arg0: Slice ): Unit {
                    return cb(PrimitiveArrayTools.getUByteArray(arg0));
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f_Native()
            cb_wrap.run_callback = callback;
            cb_wrap.data_ = DiplomatJVMRuntime.buildRustCookie(cb_wrap as Object);
            return DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f(cb_wrap)
        }
    }
}
class CallbackWrapper internal constructor (
    internal val nativeStruct: CallbackWrapperNative) {
    val cantBeEmpty: Boolean = nativeStruct.cantBeEmpty > 0

    companion object {
        internal val libClass: Class<CallbackWrapperLib> = CallbackWrapperLib::class.java
        internal val lib: CallbackWrapperLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CallbackWrapperNative::class.java).toLong()
        @JvmStatic
        
        fun testMultiArgCallback(f: (Int)->Int, x: Int): Int {
            
            val returnVal = lib.CallbackWrapper_test_multi_arg_callback(DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f.fromCallback(f).nativeStruct, x);
            return (returnVal)
        }
        @JvmStatic
        
        fun testNoArgs(h: ()->Unit): Int {
            
            val returnVal = lib.CallbackWrapper_test_no_args(DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h.fromCallback(h).nativeStruct);
            return (returnVal)
        }
        @JvmStatic
        
        fun testCbWithStruct(f: (CallbackTestingStruct)->Int): Int {
            
            val returnVal = lib.CallbackWrapper_test_cb_with_struct(DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f.fromCallback(f).nativeStruct);
            return (returnVal)
        }
        @JvmStatic
        
        fun testMultipleCbArgs(f: ()->Int, g: (Int)->Int): Int {
            
            val returnVal = lib.CallbackWrapper_test_multiple_cb_args(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f.fromCallback(f).nativeStruct, DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g.fromCallback(g).nativeStruct);
            return (returnVal)
        }
        @JvmStatic
        
        fun testSliceCbArg(arg: UByteArray, f: (UByteArray)->Unit): Unit {
            val (argMem, argSlice) = PrimitiveArrayTools.borrow(arg)
            
            val returnVal = lib.CallbackWrapper_test_slice_cb_arg(argSlice, DiplomatCallback_CallbackWrapper_test_slice_cb_arg_diplomatCallback_f.fromCallback(f).nativeStruct);
            
        }
    }

}
