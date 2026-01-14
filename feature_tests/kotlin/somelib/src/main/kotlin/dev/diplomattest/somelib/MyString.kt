package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface MyStringLib: Library {
    fun MyString_destroy(handle: Pointer)
    fun MyString_new(v: Slice): Pointer
    fun MyString_new_unsafe(v: Slice): Pointer
    fun MyString_new_owned(v: Slice): Pointer
    fun MyString_new_from_first(v: Slice): Pointer
    fun MyString_set_str(handle: Pointer, newStr: Slice): Unit
    fun MyString_get_str(handle: Pointer, write: Pointer): Unit
    fun MyString_get_static_str(): Slice
    fun MyString_string_transform(foo: Slice, write: Pointer): Unit
    fun MyString_borrow(handle: Pointer): Slice
}

class MyString internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class MyStringCleaner(val handle: Pointer, val lib: MyStringLib) : Runnable {
        override fun run() {
            lib.MyString_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<MyStringLib> = MyStringLib::class.java
        internal val lib: MyStringLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(v: String): MyString {
            val vSliceMemory = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.MyString_new(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MyString(handle, selfEdges)
            CLEANER.register(returnOpaque, MyString.MyStringCleaner(handle, MyString.lib));
            vSliceMemory?.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun newUnsafe(v: String): MyString {
            val vSliceMemory = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.MyString_new_unsafe(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MyString(handle, selfEdges)
            CLEANER.register(returnOpaque, MyString.MyStringCleaner(handle, MyString.lib));
            vSliceMemory?.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun newOwned(v: String): MyString {
            val vSliceMemory = PrimitiveArrayTools.moveUtf8(v)
            
            val returnVal = lib.MyString_new_owned(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MyString(handle, selfEdges)
            CLEANER.register(returnOpaque, MyString.MyStringCleaner(handle, MyString.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun newFromFirst(v: Array<String>): MyString {
            val vSliceMemory = PrimitiveArrayTools.borrowUtf8s(v)
            
            val returnVal = lib.MyString_new_from_first(vSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MyString(handle, selfEdges)
            CLEANER.register(returnOpaque, MyString.MyStringCleaner(handle, MyString.lib));
            vSliceMemory?.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun getStaticStr(): String {
            
            val returnVal = lib.MyString_get_static_str();
                return PrimitiveArrayTools.getUtf8(returnVal)
        }
        @JvmStatic
        
        fun stringTransform(foo: String): String {
            val fooSliceMemory = PrimitiveArrayTools.borrowUtf8(foo)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.MyString_string_transform(fooSliceMemory.slice, write);
            
            val returnString = DW.writeToString(write)
            return returnString
        }
    }
    
    fun setStr(newStr: String): Unit {
        val newStrSliceMemory = PrimitiveArrayTools.borrowUtf8(newStr)
        
        val returnVal = lib.MyString_set_str(handle, newStrSliceMemory.slice);
        
    }
    
    fun getStr(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.MyString_get_str(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    fun borrow(): String {
        
        val returnVal = lib.MyString_borrow(handle);
            return PrimitiveArrayTools.getUtf8(returnVal)
    }

}