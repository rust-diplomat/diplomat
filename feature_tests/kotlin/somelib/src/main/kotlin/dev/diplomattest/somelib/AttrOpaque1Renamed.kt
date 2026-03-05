package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface AttrOpaque1RenamedLib: Library {
    fun namespace_AttrOpaque1_destroy(handle: Pointer)
    fun namespace_AttrOpaque1_new_overload(i: Int): Pointer
    fun namespace_AttrOpaque1_new(): Pointer
    fun namespace_AttrOpaque1_mac_test(): Int
    fun namespace_AttrOpaque1_hello(): Int
    fun namespace_AttrOpaque1_method(handle: Pointer): FFIUint8
    fun renamed_on_abi_only(handle: Pointer): FFIUint8
    fun namespace_AttrOpaque1_use_unnamespaced(handle: Pointer, un: Pointer): Unit
    fun namespace_AttrOpaque1_use_namespaced(handle: Pointer, n: Int): Unit
}
internal interface AttrOpaque1RenamedInterface {
    fun method(): UByte
    fun abirenamed(): UByte
    fun useUnnamespaced(un: Unnamespaced): Unit
    fun useNamespaced(n: RenamedAttrEnum): Unit
}
/** Some example docs
*Back to all docs
*/
class AttrOpaque1Renamed internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
): AttrOpaque1RenamedInterface  {

    internal class AttrOpaque1RenamedCleaner(val handle: Pointer, val lib: AttrOpaque1RenamedLib) : Runnable {
        override fun run() {
            lib.namespace_AttrOpaque1_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<AttrOpaque1RenamedLib> = AttrOpaque1RenamedLib::class.java
        internal val lib: AttrOpaque1RenamedLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun newOverload(i: Int): AttrOpaque1Renamed {
            
            val returnVal = lib.namespace_AttrOpaque1_new_overload(i);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = AttrOpaque1Renamed(handle, selfEdges)
            CLEANER.register(returnOpaque, AttrOpaque1Renamed.AttrOpaque1RenamedCleaner(handle, AttrOpaque1Renamed.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** More example docs
        */
        fun new_(): AttrOpaque1Renamed {
            
            val returnVal = lib.namespace_AttrOpaque1_new();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = AttrOpaque1Renamed(handle, selfEdges)
            CLEANER.register(returnOpaque, AttrOpaque1Renamed.AttrOpaque1RenamedCleaner(handle, AttrOpaque1Renamed.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun macTest(): Int {
            
            val returnVal = lib.namespace_AttrOpaque1_mac_test();
            return (returnVal)
        }
        @JvmStatic
        
        fun hello(): Int {
            
            val returnVal = lib.namespace_AttrOpaque1_hello();
            return (returnVal)
        }
    }
    
    override fun method(): UByte {
        
        val returnVal = lib.namespace_AttrOpaque1_method(handle);
        return (returnVal.toUByte())
    }
    
    override fun abirenamed(): UByte {
        
        val returnVal = lib.renamed_on_abi_only(handle);
        return (returnVal.toUByte())
    }
    
    override fun useUnnamespaced(un: Unnamespaced): Unit {
        
        val returnVal = lib.namespace_AttrOpaque1_use_unnamespaced(handle, un.handle);
        
    }
    
    override fun useNamespaced(n: RenamedAttrEnum): Unit {
        
        val returnVal = lib.namespace_AttrOpaque1_use_namespaced(handle, n.toNative());
        
    }

}