package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OptionOpaqueLib: Library {
    fun OptionOpaque_destroy(handle: Pointer)
    fun OptionOpaque_new(i: Int): Pointer?
    fun OptionOpaque_new_none(): Pointer?
    fun OptionOpaque_returns(): OptionOptionStructNative
    fun OptionOpaque_option_isize(handle: Pointer): OptionFFIIsizet
    fun OptionOpaque_option_usize(handle: Pointer): OptionFFISizet
    fun OptionOpaque_option_i32(handle: Pointer): OptionInt
    fun OptionOpaque_option_u32(handle: Pointer): OptionFFIUint32
    fun OptionOpaque_new_struct(): OptionStructNative
    fun OptionOpaque_new_struct_nones(): OptionStructNative
    fun OptionOpaque_returns_none_self(handle: Pointer): Pointer?
    fun OptionOpaque_returns_some_self(handle: Pointer): Pointer?
    fun OptionOpaque_assert_integer(handle: Pointer, i: Int): Unit
    fun OptionOpaque_option_opaque_argument(arg: Pointer?): Byte
}

class OptionOpaque internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class OptionOpaqueCleaner(val handle: Pointer, val lib: OptionOpaqueLib) : Runnable {
        override fun run() {
            lib.OptionOpaque_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OptionOpaqueLib> = OptionOpaqueLib::class.java
        internal val lib: OptionOpaqueLib = Native.load("somelib", libClass)
        @JvmStatic
        
        fun new_(i: Int): OptionOpaque? {
            
            val returnVal = lib.OptionOpaque_new(i);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal ?: return null
            val returnOpaque = OptionOpaque(handle, selfEdges)
            CLEANER.register(returnOpaque, OptionOpaque.OptionOpaqueCleaner(handle, OptionOpaque.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun newNone(): OptionOpaque? {
            
            val returnVal = lib.OptionOpaque_new_none();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal ?: return null
            val returnOpaque = OptionOpaque(handle, selfEdges)
            CLEANER.register(returnOpaque, OptionOpaque.OptionOpaqueCleaner(handle, OptionOpaque.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun returns(): OptionStruct? {
            
            val returnVal = lib.OptionOpaque_returns();
            
            val intermediateOption = returnVal.option() ?: return null

            val returnStruct = OptionStruct(intermediateOption)
            return returnStruct
                                    
        }
        @JvmStatic
        
        fun newStruct(): OptionStruct {
            
            val returnVal = lib.OptionOpaque_new_struct();
            
            val returnStruct = OptionStruct(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun newStructNones(): OptionStruct {
            
            val returnVal = lib.OptionOpaque_new_struct_nones();
            
            val returnStruct = OptionStruct(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun optionOpaqueArgument(arg: OptionOpaque?): Boolean {
            
            val returnVal = lib.OptionOpaque_option_opaque_argument(arg?.handle);
            return (returnVal > 0)
        }
    }
    
    fun optionIsize(): Long? {
        
        val returnVal = lib.OptionOpaque_option_isize(handle);
        return returnVal.option()?.toLong()
    }
    
    fun optionUsize(): ULong? {
        
        val returnVal = lib.OptionOpaque_option_usize(handle);
        return returnVal.option()?.toULong()
    }
    
    fun optionI32(): Int? {
        
        val returnVal = lib.OptionOpaque_option_i32(handle);
        return returnVal.option()
    }
    
    fun optionU32(): UInt? {
        
        val returnVal = lib.OptionOpaque_option_u32(handle);
        return returnVal.option()?.toUInt()
    }
    
    fun returnsNoneSelf(): OptionOpaque? {
        
        val returnVal = lib.OptionOpaque_returns_none_self(handle);
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal ?: return null
        val returnOpaque = OptionOpaque(handle, selfEdges)
        return returnOpaque
    }
    
    fun returnsSomeSelf(): OptionOpaque? {
        
        val returnVal = lib.OptionOpaque_returns_some_self(handle);
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal ?: return null
        val returnOpaque = OptionOpaque(handle, selfEdges)
        return returnOpaque
    }
    
    fun assertInteger(i: Int): Unit {
        
        val returnVal = lib.OptionOpaque_assert_integer(handle, i);
        
    }

}