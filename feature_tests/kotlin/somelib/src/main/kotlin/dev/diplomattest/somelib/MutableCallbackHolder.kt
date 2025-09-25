package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MutableCallbackHolderLib: Library {
    fun MutableCallbackHolder_destroy(handle: Pointer)
    fun MutableCallbackHolder_new(func: DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func_Native): Pointer
    fun MutableCallbackHolder_call(handle: Pointer, a: Int): Int
}
internal interface Runner_DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func: Callback {
    fun invoke(lang_specific_context: Pointer?, arg0: Int ): Int
}

internal class DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var data_: Pointer = Pointer(0L);
    @JvmField
    internal var run_callback: Runner_DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func
        = object :  Runner_DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func {
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

internal class DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func internal constructor (
    internal val nativeStruct: DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func_Native) {
    val data_: Pointer = nativeStruct.data_
    val run_callback: Callback = nativeStruct.run_callback
    val destructor: Callback = nativeStruct.destructor

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func_Native::class.java).toLong()

        fun fromCallback(cb: (Int)->Int): DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func {
            val callback: Runner_DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func = object :  Runner_DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func {
                override fun invoke(lang_specific_context: Pointer?, arg0: Int ): Int {
                    return cb(arg0);
                }
            }
            val cb_wrap = DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func_Native()
            cb_wrap.run_callback = callback;
            cb_wrap.data_ = DiplomatJVMRuntime.buildRustCookie(cb_wrap as Object);
            return DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func(cb_wrap)
        }
    }
}


class MutableCallbackHolder internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class MutableCallbackHolderCleaner(val handle: Pointer, val lib: MutableCallbackHolderLib) : Runnable {
        override fun run() {
            lib.MutableCallbackHolder_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<MutableCallbackHolderLib> = MutableCallbackHolderLib::class.java
        internal val lib: MutableCallbackHolderLib = Native.load("somelib", libClass)
        @JvmStatic
        
        fun new_(func: (Int)->Int): MutableCallbackHolder {
            
            val returnVal = lib.MutableCallbackHolder_new(DiplomatCallback_MutableCallbackHolder_new_diplomatCallback_func.fromCallback(func).nativeStruct);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MutableCallbackHolder(handle, selfEdges)
            CLEANER.register(returnOpaque, MutableCallbackHolder.MutableCallbackHolderCleaner(handle, MutableCallbackHolder.lib));
            return returnOpaque
        }
    }
    
    fun call(a: Int): Int {
        
        val returnVal = lib.MutableCallbackHolder_call(handle, a);
        return (returnVal)
    }

}