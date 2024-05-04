package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface MyStringLib: Library {
    fun MyString_destroy(handle: Pointer)
    fun MyString_new(v: Slice): Pointer
    fun MyString_new_unsafe(v: Slice): Pointer
    fun MyString_new_owned(v: Slice): Pointer
    fun MyString_new_from_first(v: Slice): Pointer
    fun MyString_set_str(handle: Pointer, newStr: Slice): Unit
    fun MyString_get_str(handle: Pointer, writeable: Pointer): Unit
    fun MyString_get_boxed_str(handle: Pointer): Slice
}

class MyString internal constructor (
    internal val handle: Pointer,

    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>) {

    internal class MyStringCleaner(val handle: Pointer, val lib: MyStringLib) : Runnable {
        override fun run() {
            lib.MyString_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<MyStringLib> = MyStringLib::class.java
        internal val lib: MyStringLib = Native.load("somelib", libClass)
        
        fun new_(v: String): MyString {
            val (vMem, vSlice) = PrimitiveArrayTools.readUtf8(v)
            val returnVal = lib.MyString_new(vSlice);
        val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MyString(handle, selfEdges)
            CLEANER.register(returnOpaque, MyString.MyStringCleaner(handle, MyString.lib));
            vMem.close()
            return returnOpaque
        }
        
        fun newUnsafe(v: String): MyString {
            val (vMem, vSlice) = PrimitiveArrayTools.readUtf8(v)
            val returnVal = lib.MyString_new_unsafe(vSlice);
        val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MyString(handle, selfEdges)
            CLEANER.register(returnOpaque, MyString.MyStringCleaner(handle, MyString.lib));
            vMem.close()
            return returnOpaque
        }
        
        fun newOwned(v: String): MyString {
            val (vMem, vSlice) = PrimitiveArrayTools.readUtf8(v)
            val returnVal = lib.MyString_new_owned(vSlice);
        val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MyString(handle, selfEdges)
            CLEANER.register(returnOpaque, MyString.MyStringCleaner(handle, MyString.lib));
            
            return returnOpaque
        }
        
        fun newFromFirst(v: Array<String>): MyString {
            val (vMem, vSlice) = PrimitiveArrayTools.readUtf8s(v)
            val returnVal = lib.MyString_new_from_first(vSlice);
        val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = MyString(handle, selfEdges)
            CLEANER.register(returnOpaque, MyString.MyStringCleaner(handle, MyString.lib));
            vMem.forEach {it.close()}
            return returnOpaque
        }
    }
    
    fun setStr(newStr: String): Unit {
        val (newStrMem, newStrSlice) = PrimitiveArrayTools.readUtf8(newStr)
        val returnVal = lib.MyString_set_str(handle, newStrSlice);
    
    }
    
    fun getStr(): String {
        val writeable = DW.lib.diplomat_buffer_writeable_create(0)
        val returnVal = lib.MyString_get_str(handle, writeable);
    
        val returnString = DW.writeableToString(writeable)
        DW.lib.diplomat_buffer_writeable_destroy(writeable)
        return returnString
    }
    
    fun getBoxedStr(): String {
        val returnVal = lib.MyString_get_boxed_str(handle);
        val string = PrimitiveArrayTools.getUtf8(returnVal)
        Native.free(Pointer.nativeValue(returnVal.data))
        return string
    }

}
