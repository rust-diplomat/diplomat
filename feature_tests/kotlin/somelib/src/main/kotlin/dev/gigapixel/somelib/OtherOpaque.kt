package dev.gigapixel.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


interface OtherOpaqueLib: Library {
    fun OtherOpaque_destroy(handle: Long)
    fun OtherOpaque_from_usize(number: Long): Long
    fun OtherOpaque_change(handle: Long, number: Long): Unit
    fun OtherOpaque_borrow(handle: Long): Long
    fun OtherOpaque_borrow_other(other: Long): Long
    fun OtherOpaque_borrow_self_or_other(handle: Long, other: Long): Long
    fun OtherOpaque_get_len_and_add(handle: Long, other: Long): Long
    fun OtherOpaque_dummy_str(handle: Long): Slice
    fun OtherOpaque_wrapper(handle: Long): Long
}

class OtherOpaque internal constructor (
    internal val handle: Long,
    internal val selfEdges: List<Any>) {

    internal class OtherOpaqueCleaner(val handle: Long, val lib: OtherOpaqueLib) : Runnable {
        override fun run() {
            lib.OtherOpaque_destroy(handle)
        }
    }

    companion object {
        val libClass: Class<OtherOpaqueLib> = OtherOpaqueLib::class.java
        val lib: OtherOpaqueLib = Native.load("somelib", libClass)
        fun fromUsize(number: Long): OtherOpaque {
        
            
            val returnVal = lib.OtherOpaque_from_usize(number);
        
            val selfEdges: List<Any> = listOf()
            val handle = returnVal
            val returnOpaque = OtherOpaque(handle, selfEdges)
            CLEANER.register(returnOpaque, OtherOpaque.OtherOpaqueCleaner(handle, OtherOpaque.lib));
            
            return returnOpaque
        
        }
        fun borrowOther(other: OtherOpaque): OtherOpaque {
        
            
            val returnVal = lib.OtherOpaque_borrow_other(other.handle);
        
            val selfEdges: List<Any> = listOf()
            val handle = returnVal
            val returnOpaque = OtherOpaque(handle, selfEdges)
            
            return returnOpaque
        
        }
    }
    fun change(number: Long): Unit {
    
        
        val returnVal = lib.OtherOpaque_change(handle, number);
    }
    fun borrow(): OtherOpaque {
    
        
        val returnVal = lib.OtherOpaque_borrow(handle);
    
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal
        val returnOpaque = OtherOpaque(handle, selfEdges)
        
        return returnOpaque
    
    }
    fun borrowSelfOrOther(other: OtherOpaque): OtherOpaque {
    
        
        val returnVal = lib.OtherOpaque_borrow_self_or_other(handle, other.handle);
    
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal
        val returnOpaque = OtherOpaque(handle, selfEdges)
        
        return returnOpaque
    
    }
    fun getLenAndAdd(other: Long): Long {
    
        
        val returnVal = lib.OtherOpaque_get_len_and_add(handle, other);
    return returnVal
    }
    fun dummyStr(): String {
    
        
        val returnVal = lib.OtherOpaque_dummy_str(handle);
        return PrimitiveArrayTools.getUtf8(returnVal)
    }
    fun wrapper(): Utf16Wrap {
    
        
        val returnVal = lib.OtherOpaque_wrapper(handle);
    
        val selfEdges: List<Any> = listOf()
        val handle = returnVal
        val returnOpaque = Utf16Wrap(handle, selfEdges)
        CLEANER.register(returnOpaque, Utf16Wrap.Utf16WrapCleaner(handle, Utf16Wrap.lib));
        
        return returnOpaque
    
    }

}