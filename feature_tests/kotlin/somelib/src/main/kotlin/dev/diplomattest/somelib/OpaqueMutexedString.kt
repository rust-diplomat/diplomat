package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OpaqueMutexedStringLib: Library {
    fun OpaqueMutexedString_destroy(handle: Pointer)
    fun OpaqueMutexedString_from_usize(number: FFISizet): Pointer
    fun OpaqueMutexedString_change(handle: Pointer, number: FFISizet): Unit
    fun OpaqueMutexedString_borrow(handle: Pointer): Pointer
    fun OpaqueMutexedString_borrow_other(other: Pointer): Pointer
    fun OpaqueMutexedString_borrow_self_or_other(handle: Pointer, other: Pointer): Pointer
    fun OpaqueMutexedString_get_len_and_add(handle: Pointer, other: FFISizet): FFISizet
    fun OpaqueMutexedString_dummy_str(handle: Pointer): Slice
    fun OpaqueMutexedString_wrapper(handle: Pointer): Pointer
    fun OpaqueMutexedString_to_unsigned_from_unsigned(handle: Pointer, input: FFIUint16): FFIUint16
}

class OpaqueMutexedString internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class OpaqueMutexedStringCleaner(val handle: Pointer, val lib: OpaqueMutexedStringLib) : Runnable {
        override fun run() {
            lib.OpaqueMutexedString_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OpaqueMutexedStringLib> = OpaqueMutexedStringLib::class.java
        internal val lib: OpaqueMutexedStringLib = Native.load("somelib", libClass)
        @JvmStatic
        
        fun fromUsize(number: ULong): OpaqueMutexedString {
            
            val returnVal = lib.OpaqueMutexedString_from_usize(FFISizet(number));
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = OpaqueMutexedString(handle, selfEdges)
            CLEANER.register(returnOpaque, OpaqueMutexedString.OpaqueMutexedStringCleaner(handle, OpaqueMutexedString.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun borrowOther(other: OpaqueMutexedString): OpaqueMutexedString {
            
            val returnVal = lib.OpaqueMutexedString_borrow_other(other.handle);
            val selfEdges: List<Any> = listOf(other)
            val handle = returnVal 
            val returnOpaque = OpaqueMutexedString(handle, selfEdges)
            return returnOpaque
        }
    }
    
    fun change(number: ULong): Unit {
        
        val returnVal = lib.OpaqueMutexedString_change(handle, FFISizet(number));
        
    }
    
    fun borrow(): OpaqueMutexedString {
        
        val returnVal = lib.OpaqueMutexedString_borrow(handle);
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal 
        val returnOpaque = OpaqueMutexedString(handle, selfEdges)
        return returnOpaque
    }
    
    fun borrowSelfOrOther(other: OpaqueMutexedString): OpaqueMutexedString {
        
        val returnVal = lib.OpaqueMutexedString_borrow_self_or_other(handle, other.handle);
        val selfEdges: List<Any> = listOf(this) + listOf(other)
        val handle = returnVal 
        val returnOpaque = OpaqueMutexedString(handle, selfEdges)
        return returnOpaque
    }
    
    fun getLenAndAdd(other: ULong): ULong {
        
        val returnVal = lib.OpaqueMutexedString_get_len_and_add(handle, FFISizet(other));
        return (returnVal.toULong())
    }
    
    fun dummyStr(): String {
        
        val returnVal = lib.OpaqueMutexedString_dummy_str(handle);
            return PrimitiveArrayTools.getUtf8(returnVal)
    }
    
    fun wrapper(): Utf16Wrap {
        
        val returnVal = lib.OpaqueMutexedString_wrapper(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = Utf16Wrap(handle, selfEdges)
        CLEANER.register(returnOpaque, Utf16Wrap.Utf16WrapCleaner(handle, Utf16Wrap.lib));
        return returnOpaque
    }
    
    fun toUnsignedFromUnsigned(input: UShort): UShort {
        
        val returnVal = lib.OpaqueMutexedString_to_unsigned_from_unsigned(handle, FFIUint16(input));
        return (returnVal.toUShort())
    }

}