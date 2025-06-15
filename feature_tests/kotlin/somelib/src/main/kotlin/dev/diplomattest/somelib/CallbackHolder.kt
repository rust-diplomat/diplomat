package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CallbackHolderLib: Library {
    fun CallbackHolder_destroy(handle: Pointer)
    fun CallbackHolder_new(func: DiplomatCallback_CallbackHolder_new_diplomatCallback_func_Native): Pointer
    fun CallbackHolder_call(handle: Pointer, a: Int): Int
}
internal interface Runner_DiplomatCallback_CallbackHolder_new_diplomatCallback_func: Callback {
    fun invoke(lang_specific_context: Pointer?, arg0: Int ): Int
}

internal class DiplomatCallback_CallbackHolder_new_diplomatCallback_func_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var data_: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_CallbackHolder_new_diplomatCallback_func
        = object :  Runner_DiplomatCallback_CallbackHolder_new_diplomatCallback_func {
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

internal class DiplomatCallback_CallbackHolder_new_diplomatCallback_func internal constructor (
    internal val nativeStruct: DiplomatCallback_CallbackHolder_new_diplomatCallback_func_Native) {
    val data_: Pointer = nativeStruct.data_
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Callback = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_CallbackHolder_new_diplomatCallback_func_Native::class.java).toLong()

        fun fromCallback(cb: (Int)->Int): DiplomatCallback_CallbackHolder_new_diplomatCallback_func {
            val callback: Runner_DiplomatCallback_CallbackHolder_new_diplomatCallback_func = object :  Runner_DiplomatCallback_CallbackHolder_new_diplomatCallback_func {
                override fun invoke(lang_specific_context: Pointer?, arg0: Int ): Int {
                    return cb(arg0);
                }
            }
            val cb_wrap = DiplomatCallback_CallbackHolder_new_diplomatCallback_func_Native()
            cb_wrap.run_callback = callback;
            cb_wrap.data_ = DiplomatJVMRuntime.buildRustCookie(cb_wrap as Object);
            return DiplomatCallback_CallbackHolder_new_diplomatCallback_func(cb_wrap)
        }
    }
}


class CallbackHolder internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class CallbackHolderCleaner(val handle: Pointer, val lib: CallbackHolderLib) : Runnable {
        override fun run() {
            lib.CallbackHolder_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<CallbackHolderLib> = CallbackHolderLib::class.java
        internal val lib: CallbackHolderLib = Native.load("somelib", libClass)
        @JvmStatic
        
        fun new_(func: (Int)->Int): CallbackHolder {
            
            val returnVal = lib.CallbackHolder_new(DiplomatCallback_CallbackHolder_new_diplomatCallback_func.fromCallback(func).nativeStruct);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CallbackHolder(handle, selfEdges)
            CLEANER.register(returnOpaque, CallbackHolder.CallbackHolderCleaner(handle, CallbackHolder.lib));
            return returnOpaque
        }
    }
    
    fun call(a: Int): Int {
        
        val returnVal = lib.CallbackHolder_call(handle, a);
        return (returnVal)
    }

}