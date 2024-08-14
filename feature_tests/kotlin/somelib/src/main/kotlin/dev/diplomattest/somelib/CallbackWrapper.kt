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
    fun invoke(ignored: Pointer?, arg0: Int ): Int
}

class DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var dataa: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f {
                override fun invoke(ignored: Pointer?, arg0: Int ): Int {
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

class DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f_Native) {
    val dataa: Pointer = nativeStruct.dataa
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Pointer = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f_Native::class.java).toLong()
        
        fun fromCallback(cb: (Int)->Int): DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f = object :  Runner_DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f {
                override fun invoke(ignored: Pointer?, arg0: Int ): Int {
                    return cb(arg0);
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f_Native()
            cb_wrap.run_callback = callback;
            return DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f(cb_wrap)
        }
    }
}
internal interface Runner_DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h: Callback {
    fun invoke(ignored: Pointer?): Void
}

class DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var dataa: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h {
                override fun invoke(ignored: Pointer?): Void {
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

class DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h_Native) {
    val dataa: Pointer = nativeStruct.dataa
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Pointer = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h_Native::class.java).toLong()
        
        fun fromCallback(cb: ()->Void): DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h = object :  Runner_DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h {
                override fun invoke(ignored: Pointer?): Void {
                    return cb();
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h_Native()
            cb_wrap.run_callback = callback;
            return DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h(cb_wrap)
        }
    }
}
internal interface Runner_DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f: Callback {
    fun invoke(ignored: Pointer?, arg0: CallbackTestingStruct ): Int
}

class DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var dataa: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f {
                override fun invoke(ignored: Pointer?, arg0: CallbackTestingStruct ): Int {
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

class DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f_Native) {
    val dataa: Pointer = nativeStruct.dataa
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Pointer = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f_Native::class.java).toLong()
        
        fun fromCallback(cb: (CallbackTestingStruct)->Int): DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f = object :  Runner_DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f {
                override fun invoke(ignored: Pointer?, arg0: CallbackTestingStruct ): Int {
                    return cb(arg0);
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f_Native()
            cb_wrap.run_callback = callback;
            return DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f(cb_wrap)
        }
    }
}
internal interface Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f: Callback {
    fun invoke(ignored: Pointer?): Int
}

class DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var dataa: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f {
                override fun invoke(ignored: Pointer?): Int {
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

class DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f_Native) {
    val dataa: Pointer = nativeStruct.dataa
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Pointer = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f_Native::class.java).toLong()
        
        fun fromCallback(cb: ()->Int): DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f = object :  Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f {
                override fun invoke(ignored: Pointer?): Int {
                    return cb();
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f_Native()
            cb_wrap.run_callback = callback;
            return DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f(cb_wrap)
        }
    }
}
internal interface Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g: Callback {
    fun invoke(ignored: Pointer?, arg0: Int ): Int
}

class DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var dataa: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g
        = object :  Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g {
                override fun invoke(ignored: Pointer?, arg0: Int ): Int {
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

class DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g_Native) {
    val dataa: Pointer = nativeStruct.dataa
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Pointer = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g_Native::class.java).toLong()
        
        fun fromCallback(cb: (Int)->Int): DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g {
            val callback: Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g = object :  Runner_DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g {
                override fun invoke(ignored: Pointer?, arg0: Int ): Int {
                    return cb(arg0);
                }
            }
            val cb_wrap = DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g_Native()
            cb_wrap.run_callback = callback;
            return DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g(cb_wrap)
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
        
        fun testMultiArgCallback(f: DiplomatCallback_CallbackWrapper_test_multi_arg_callback_diplomatCallback_f, x: Int): Int {
            
            val returnVal = lib.CallbackWrapper_test_multi_arg_callback(f.nativeStruct, x);
            return returnVal
        }
        
        fun testNoArgs(h: DiplomatCallback_CallbackWrapper_test_no_args_diplomatCallback_h): Int {
            
            val returnVal = lib.CallbackWrapper_test_no_args(h.nativeStruct);
            return returnVal
        }
        
        fun testCbWithStruct(f: DiplomatCallback_CallbackWrapper_test_cb_with_struct_diplomatCallback_f): Int {
            
            val returnVal = lib.CallbackWrapper_test_cb_with_struct(f.nativeStruct);
            return returnVal
        }
        
        fun testMultipleCbArgs(f: DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_f, g: DiplomatCallback_CallbackWrapper_test_multiple_cb_args_diplomatCallback_g): Int {
            
            val returnVal = lib.CallbackWrapper_test_multiple_cb_args(f.nativeStruct, g.nativeStruct);
            return returnVal
        }
    }

}
