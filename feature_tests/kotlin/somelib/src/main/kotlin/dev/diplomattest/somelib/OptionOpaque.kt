package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface OptionOpaqueLib: Library {
    fun OptionOpaque_destroy(handle: Pointer)
    fun OptionOpaque_new(i: Int): Pointer?
    fun OptionOpaque_new_none(): Pointer?
    fun OptionOpaque_returns(): OptionStructNative?
    fun OptionOpaque_new_struct(): OptionStructNative
    fun OptionOpaque_new_struct_nones(): OptionStructNative
    fun OptionOpaque_assert_integer(handle: Pointer, i: Int): Unit
    fun OptionOpaque_option_opaque_argument(arg: Pointer?): Boolean
}

class OptionOpaque internal constructor (
    internal val handle: Pointer,

    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>) {

    internal class OptionOpaqueCleaner(val handle: Pointer, val lib: OptionOpaqueLib) : Runnable {
        override fun run() {
            lib.OptionOpaque_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OptionOpaqueLib> = OptionOpaqueLib::class.java
        internal val lib: OptionOpaqueLib = Native.load("somelib", libClass)
        
        fun new_(i: Int): OptionOpaque? {
            val returnVal = lib.OptionOpaque_new(i);
        val selfEdges: List<Any> = listOf()
            val handle = returnVal ?: return null
            val returnOpaque = OptionOpaque(handle, selfEdges)
            CLEANER.register(returnOpaque, OptionOpaque.OptionOpaqueCleaner(handle, OptionOpaque.lib));
            
            return returnOpaque
        }
        
        fun newNone(): OptionOpaque? {
            val returnVal = lib.OptionOpaque_new_none();
        val selfEdges: List<Any> = listOf()
            val handle = returnVal ?: return null
            val returnOpaque = OptionOpaque(handle, selfEdges)
            CLEANER.register(returnOpaque, OptionOpaque.OptionOpaqueCleaner(handle, OptionOpaque.lib));
            
            return returnOpaque
        }
        
        fun returns(): OptionStruct? {
            val returnVal = lib.OptionOpaque_returns();
        
            if (returnVal == null) {
                return null
            } else {
                
            val returnStruct = OptionStruct(returnVal)
            return returnStruct
            }
        }
        
        fun newStruct(): OptionStruct {
            val returnVal = lib.OptionOpaque_new_struct();
        
            val returnStruct = OptionStruct(returnVal)
            return returnStruct
        }
        
        fun newStructNones(): OptionStruct {
            val returnVal = lib.OptionOpaque_new_struct_nones();
        
            val returnStruct = OptionStruct(returnVal)
            return returnStruct
        }
        
        fun optionOpaqueArgument(arg: OptionOpaque?): Boolean {
            val returnVal = lib.OptionOpaque_option_opaque_argument(arg?.handle);
        return returnVal
        }
    }
    
    fun assertInteger(i: Int): Unit {
        val returnVal = lib.OptionOpaque_assert_integer(handle, i);
    
    }

}
