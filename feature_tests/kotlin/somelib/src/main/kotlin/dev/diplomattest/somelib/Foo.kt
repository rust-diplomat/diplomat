package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface FooLib: Library {
    fun Foo_destroy(handle: Pointer)
    fun Foo_new(x: Slice): Pointer
    fun Foo_get_bar(handle: Pointer): Pointer
    fun Foo_new_static(x: Slice): Pointer
    fun Foo_as_returning(handle: Pointer): BorrowedFieldsReturningNative
    fun Foo_extract_from_fields(fields: BorrowedFieldsNative): Pointer
    fun Foo_extract_from_bounds(bounds: BorrowedFieldsWithBoundsNative, anotherString: Slice): Pointer
}

class Foo internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
)  {

    internal class FooCleaner(val handle: Pointer, val lib: FooLib) : Runnable {
        override fun run() {
            lib.Foo_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<FooLib> = FooLib::class.java
        internal val lib: FooLib = Native.load("somelib", libClass)
        @JvmStatic
        
        fun new_(x: String): Foo {
            val (xMem, xSlice) = PrimitiveArrayTools.borrowUtf8(x)
            
            val returnVal = lib.Foo_new(xSlice);
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any?> = listOf(xMem)
            val handle = returnVal 
            val returnOpaque = Foo(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, Foo.FooCleaner(handle, Foo.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun newStatic(x: String): Foo {
            val (xMem, xSlice) = PrimitiveArrayTools.borrowUtf8(x)
            
            val returnVal = lib.Foo_new_static(xSlice);
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any?> = listOf()
            val handle = returnVal 
            val returnOpaque = Foo(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, Foo.FooCleaner(handle, Foo.lib));
            if (xMem != null) xMem.close()
            return returnOpaque
        }
        @JvmStatic
        
        fun extractFromFields(fields: BorrowedFields): Foo {
            
            val returnVal = lib.Foo_extract_from_fields(fields.nativeStruct);
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any?> = fields.aEdges
            val handle = returnVal 
            val returnOpaque = Foo(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, Foo.FooCleaner(handle, Foo.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Test that the extraction logic correctly pins the right fields
        */
        fun extractFromBounds(bounds: BorrowedFieldsWithBounds, anotherString: String): Foo {
            val (anotherStringMem, anotherStringSlice) = PrimitiveArrayTools.borrowUtf8(anotherString)
            
            val returnVal = lib.Foo_extract_from_bounds(bounds.nativeStruct, anotherStringSlice);
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any?> = bounds.bEdges + bounds.cEdges + listOf(anotherStringMem)
            val handle = returnVal 
            val returnOpaque = Foo(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, Foo.FooCleaner(handle, Foo.lib));
            return returnOpaque
        }
    }
    
    fun getBar(): Bar {
        
        val returnVal = lib.Foo_get_bar(handle);
        val selfEdges: List<Any> = listOf()
        val bEdges: List<Any?> = listOf(this)
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = Bar(handle, selfEdges, bEdges, aEdges)
        CLEANER.register(returnOpaque, Bar.BarCleaner(handle, Bar.lib));
        return returnOpaque
    }
    
    fun asReturning(): BorrowedFieldsReturning {
        
        val returnVal = lib.Foo_as_returning(handle);
        
        val aEdges: List<Any?> = listOf(this)
        val returnStruct = BorrowedFieldsReturning(returnVal, aEdges)
        return returnStruct
    }

}