package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OpaqueLib: Library {
    fun Opaque_destroy(handle: Pointer)
    fun Opaque_new(): Pointer
    fun Opaque_try_from_utf8(input: Slice): Pointer?
    fun Opaque_from_str(input: Slice): Pointer
    fun Opaque_get_debug_str(handle: Pointer, write: Pointer): Unit
    fun Opaque_assert_struct(handle: Pointer, s: MyStructNative): Unit
    fun Opaque_returns_usize(): FFISizet
    fun Opaque_returns_imported(): ImportedStructNative
    fun Opaque_cmp(): Byte
}

class Opaque internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class OpaqueCleaner(val handle: Pointer, val lib: OpaqueLib) : Runnable {
        override fun run() {
            lib.Opaque_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OpaqueLib> = OpaqueLib::class.java
        internal val lib: OpaqueLib = Native.load("somelib", libClass)
        @JvmStatic
        
        fun new_(): Opaque {
            
            val returnVal = lib.Opaque_new();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Opaque(handle, selfEdges)
            CLEANER.register(returnOpaque, Opaque.OpaqueCleaner(handle, Opaque.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun tryFromUtf8(input: String): Opaque? {
            val (inputMem, inputSlice) = PrimitiveArrayTools.borrowUtf8(input)
            
            val returnVal = lib.Opaque_try_from_utf8(inputSlice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal ?: return null
            val returnOpaque = Opaque(handle, selfEdges)
            CLEANER.register(returnOpaque, Opaque.OpaqueCleaner(handle, Opaque.lib));
            if (inputMem != null) inputMem.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun fromStr(input: String): Opaque {
            val (inputMem, inputSlice) = PrimitiveArrayTools.borrowUtf8(input)
            
            val returnVal = lib.Opaque_from_str(inputSlice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Opaque(handle, selfEdges)
            CLEANER.register(returnOpaque, Opaque.OpaqueCleaner(handle, Opaque.lib));
            if (inputMem != null) inputMem.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun returnsUsize(): ULong {
            
            val returnVal = lib.Opaque_returns_usize();
            return (returnVal.toULong())
        }
        @JvmStatic
        
        fun returnsImported(): ImportedStruct {
            
            val returnVal = lib.Opaque_returns_imported();
            
            val returnStruct = ImportedStruct(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun cmp(): Byte {
            
            val returnVal = lib.Opaque_cmp();
            return (returnVal)
        }
    }
    
    fun getDebugStr(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.Opaque_get_debug_str(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** See the [Rust documentation for `something`](https://docs.rs/Something/latest/struct.Something.html#method.something) for more information.
    *
    *See the [Rust documentation for `something_else`](https://docs.rs/Something/latest/struct.Something.html#method.something_else) for more information.
    *
    *Additional information: [1](https://docs.rs/Something/latest/struct.Something.html#method.something_small), [2](https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something)
    */
    fun assertStruct(s: MyStruct): Unit {
        
        val returnVal = lib.Opaque_assert_struct(handle, s.nativeStruct);
        
    }

}