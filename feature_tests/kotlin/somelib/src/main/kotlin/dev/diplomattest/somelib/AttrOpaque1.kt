package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface AttrOpaque1Lib: Library {
    fun AttrOpaque1_destroy(handle: Pointer)
    fun namespace_AttrOpaque1_new(): Pointer
    fun namespace_AttrOpaque1_method(handle: Pointer): Byte
    fun renamed_on_abi_only(handle: Pointer): Byte
    fun namespace_AttrOpaque1_use_unnamespaced(handle: Pointer, un: Pointer): Unit
    fun namespace_AttrOpaque1_use_namespaced(handle: Pointer, n: Int): Unit
}

class AttrOpaque1 internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
)  {

    internal class AttrOpaque1Cleaner(val handle: Pointer, val lib: AttrOpaque1Lib) : Runnable {
        override fun run() {
            lib.AttrOpaque1_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<AttrOpaque1Lib> = AttrOpaque1Lib::class.java
        internal val lib: AttrOpaque1Lib = Native.load("somelib", libClass)
        
        fun totally_not_new(): AttrOpaque1Renamed {
            
            val returnVal = lib.namespace_AttrOpaque1_new();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = AttrOpaque1(handle, selfEdges)
            CLEANER.register(returnOpaque, AttrOpaque1.AttrOpaque1Cleaner(handle, AttrOpaque1.lib));
            
            return returnOpaque
        }
    }
    
    fun method_renamed(): UByte {
        
        val returnVal = lib.namespace_AttrOpaque1_method(handle);
        return returnVal.toUByte()
    }
    
    fun abirenamed(): UByte {
        
        val returnVal = lib.renamed_on_abi_only(handle);
        return returnVal.toUByte()
    }
    
    fun useUnnamespaced(un: Unnamespaced): Unit {
        
        val returnVal = lib.namespace_AttrOpaque1_use_unnamespaced(handle, un.handle);
        
    }
    
    fun useNamespaced(n: RenamedAttrEnum): Unit {
        
        val returnVal = lib.namespace_AttrOpaque1_use_namespaced(handle, n.toNative());
        
    }

}
