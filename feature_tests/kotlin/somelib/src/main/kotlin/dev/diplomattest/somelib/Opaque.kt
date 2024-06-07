package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface OpaqueLib: Library {
    fun Opaque_destroy(handle: Pointer)
    fun Opaque_new(): Pointer
    fun Opaque_assert_struct(handle: Pointer, s: MyStructNative): Unit
    fun Opaque_returns_usize(): Long
    fun Opaque_returns_imported(): ImportedStructNative
    fun Opaque_cmp(): Byte
}

class Opaque internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
)  {

    internal class OpaqueCleaner(val handle: Pointer, val lib: OpaqueLib) : Runnable {
        override fun run() {
            lib.Opaque_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OpaqueLib> = OpaqueLib::class.java
        internal val lib: OpaqueLib = Native.load("somelib", libClass)
        
        fun new_(): Opaque {
            
            val returnVal = lib.Opaque_new();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Opaque(handle, selfEdges)
            CLEANER.register(returnOpaque, Opaque.OpaqueCleaner(handle, Opaque.lib));
            
            return returnOpaque
        }
        
        fun returnsUsize(): ULong {
            
            val returnVal = lib.Opaque_returns_usize();
            return returnVal.toULong()
        }
        
        fun returnsImported(): ImportedStruct {
            
            val returnVal = lib.Opaque_returns_imported();
            
            val returnStruct = ImportedStruct(returnVal)
            return returnStruct
        }
        
        fun cmp(): Byte {
            
            val returnVal = lib.Opaque_cmp();
            return returnVal
        }
    }
    
    fun assertStruct(s: MyStruct): Unit {
        
        val returnVal = lib.Opaque_assert_struct(handle, s.nativeStruct);
        
    }

}
