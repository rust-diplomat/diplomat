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
    fun registerCleaner() {
        CLEANER.register(this, Foo.FooCleaner(handle, Foo.lib));
    }

    companion object {
        internal val libClass: Class<FooLib> = FooLib::class.java
        internal val lib: FooLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(x: String): Foo {
            // This lifetime edge depends on lifetimes: 'a
            val aEdges: MutableList<Any> = mutableListOf();
            val xSliceMemory = PrimitiveArrayTools.borrowUtf8(x).into(listOf(aEdges))
            
            val returnVal = lib.Foo_new(xSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Foo(handle, selfEdges, aEdges)
            returnOpaque.registerCleaner()
            return returnOpaque
        }
        @JvmStatic
        
        fun newStatic(x: String): Foo {
            // This lifetime edge depends on lifetimes: 'a
            val aEdges: MutableList<Any> = mutableListOf();
            val xSliceMemory = PrimitiveArrayTools.borrowUtf8(x).leakStatic()
            
            val returnVal = lib.Foo_new_static(xSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Foo(handle, selfEdges, aEdges)
            returnOpaque.registerCleaner()
            return returnOpaque
        }
        @JvmStatic
        
        fun extractFromFields(fields: BorrowedFields): Foo {
            // This lifetime edge depends on lifetimes: 'a
            val aEdges: MutableList<Any> = mutableListOf();
            
            val returnVal = lib.Foo_extract_from_fields(fields.toNative(aAppendArray = arrayOf(aEdges)));
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Foo(handle, selfEdges, aEdges)
            returnOpaque.registerCleaner()
            return returnOpaque
        }
        @JvmStatic
        
        /** Test that the extraction logic correctly pins the right fields
        */
        fun extractFromBounds(bounds: BorrowedFieldsWithBounds, anotherString: String): Foo {
            val temporaryEdgeArena: MutableList<Any> = mutableListOf()
            // This lifetime edge depends on lifetimes: 'a, 'y, 'z
            val aEdges: MutableList<Any> = mutableListOf();
            val anotherStringSliceMemory = PrimitiveArrayTools.borrowUtf8(anotherString).into(listOf(aEdges))
            
            val returnVal = lib.Foo_extract_from_bounds(bounds.toNative(aAppendArray = arrayOf(temporaryEdgeArena), bAppendArray = arrayOf(aEdges), cAppendArray = arrayOf(aEdges)), anotherStringSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Foo(handle, selfEdges, aEdges)
            returnOpaque.registerCleaner()
            return returnOpaque
        }
    }
    
    fun getBar(): Bar {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        // This lifetime edge depends on lifetimes: 'a, 'b
        val bEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.Foo_get_bar(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = Bar(handle, selfEdges, bEdges, aEdges)
        returnOpaque.registerCleaner()
        return returnOpaque
    }
    
    fun asReturning(): BorrowedFieldsReturning {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.Foo_as_returning(handle);
        val returnStruct = BorrowedFieldsReturning.fromNative(returnVal, aEdges)
        return returnStruct
    }

}