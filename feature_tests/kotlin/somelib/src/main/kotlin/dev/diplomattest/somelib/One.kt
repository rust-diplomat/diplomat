package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OneLib: Library {
    fun One_destroy(handle: Pointer)
    fun One_transitivity(hold: Pointer, nohold: Pointer): Pointer
    fun One_cycle(hold: Pointer, nohold: Pointer): Pointer
    fun One_many_dependents(a: Pointer, b: Pointer, c: Pointer, d: Pointer, nohold: Pointer): Pointer
    fun One_return_outlives_param(hold: Pointer, nohold: Pointer): Pointer
    fun One_diamond_top(top: Pointer, left: Pointer, right: Pointer, bottom: Pointer): Pointer
    fun One_diamond_left(top: Pointer, left: Pointer, right: Pointer, bottom: Pointer): Pointer
    fun One_diamond_right(top: Pointer, left: Pointer, right: Pointer, bottom: Pointer): Pointer
    fun One_diamond_bottom(top: Pointer, left: Pointer, right: Pointer, bottom: Pointer): Pointer
    fun One_diamond_and_nested_types(a: Pointer, b: Pointer, c: Pointer, d: Pointer, nohold: Pointer): Pointer
    fun One_implicit_bounds(explicitHold: Pointer, implicitHold: Pointer, nohold: Pointer): Pointer
    fun One_implicit_bounds_deep(explicit: Pointer, implicit1: Pointer, implicit2: Pointer, nohold: Pointer): Pointer
}

class One internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
)  {

    internal class OneCleaner(val handle: Pointer, val lib: OneLib) : Runnable {
        override fun run() {
            lib.One_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OneLib> = OneLib::class.java
        internal val lib: OneLib = Native.load("somelib", libClass)
        @JvmStatic
        
        fun transitivity(hold: One, nohold: One): One {
            
            val returnVal = lib.One_transitivity(hold.handle, nohold.handle);
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any?> = listOf(hold)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun cycle(hold: Two, nohold: One): One {
            
            val returnVal = lib.One_cycle(hold.handle, nohold.handle);
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any?> = listOf(hold)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun manyDependents(a: One, b: One, c: Two, d: Two, nohold: Two): One {
            
            val returnVal = lib.One_many_dependents(a.handle, b.handle, c.handle, d.handle, nohold.handle);
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any?> = listOf(a) + listOf(b) + listOf(c) + listOf(d)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun returnOutlivesParam(hold: Two, nohold: One): One {
            
            val returnVal = lib.One_return_outlives_param(hold.handle, nohold.handle);
            val selfEdges: List<Any> = listOf()
            val longEdges: List<Any?> = listOf(hold)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, longEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun diamondTop(top: One, left: One, right: One, bottom: One): One {
            
            val returnVal = lib.One_diamond_top(top.handle, left.handle, right.handle, bottom.handle);
            val selfEdges: List<Any> = listOf()
            val topEdges: List<Any?> = listOf(top) + listOf(left) + listOf(right) + listOf(bottom)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, topEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun diamondLeft(top: One, left: One, right: One, bottom: One): One {
            
            val returnVal = lib.One_diamond_left(top.handle, left.handle, right.handle, bottom.handle);
            val selfEdges: List<Any> = listOf()
            val leftEdges: List<Any?> = listOf(left) + listOf(bottom)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, leftEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun diamondRight(top: One, left: One, right: One, bottom: One): One {
            
            val returnVal = lib.One_diamond_right(top.handle, left.handle, right.handle, bottom.handle);
            val selfEdges: List<Any> = listOf()
            val rightEdges: List<Any?> = listOf(right) + listOf(bottom)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, rightEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun diamondBottom(top: One, left: One, right: One, bottom: One): One {
            
            val returnVal = lib.One_diamond_bottom(top.handle, left.handle, right.handle, bottom.handle);
            val selfEdges: List<Any> = listOf()
            val bottomEdges: List<Any?> = listOf(bottom)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, bottomEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun diamondAndNestedTypes(a: One, b: One, c: One, d: One, nohold: One): One {
            
            val returnVal = lib.One_diamond_and_nested_types(a.handle, b.handle, c.handle, d.handle, nohold.handle);
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any?> = listOf(a) + listOf(b) + listOf(c) + listOf(d)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun implicitBounds(explicitHold: One, implicitHold: One, nohold: One): One {
            
            val returnVal = lib.One_implicit_bounds(explicitHold.handle, implicitHold.handle, nohold.handle);
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any?> = listOf(explicitHold) + listOf(implicitHold)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun implicitBoundsDeep(explicit: One, implicit1: One, implicit2: One, nohold: One): One {
            
            val returnVal = lib.One_implicit_bounds_deep(explicit.handle, implicit1.handle, implicit2.handle, nohold.handle);
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any?> = listOf(explicit) + listOf(implicit1) + listOf(implicit2)
            val handle = returnVal 
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            return returnOpaque
        }
    }

}