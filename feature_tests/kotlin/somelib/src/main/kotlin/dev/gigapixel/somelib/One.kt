package dev.gigapixel.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface OneLib: Library {
    fun One_destroy(handle: Long)
    fun One_transitivity(hold: Long, nohold: Long): Long
    fun One_cycle(hold: Long, nohold: Long): Long
    fun One_many_dependents(a: Long, b: Long, c: Long, d: Long, nohold: Long): Long
    fun One_return_outlives_param(hold: Long, nohold: Long): Long
    fun One_diamond_top(top: Long, left: Long, right: Long, bottom: Long): Long
    fun One_diamond_left(top: Long, left: Long, right: Long, bottom: Long): Long
    fun One_diamond_right(top: Long, left: Long, right: Long, bottom: Long): Long
    fun One_diamond_bottom(top: Long, left: Long, right: Long, bottom: Long): Long
    fun One_diamond_and_nested_types(a: Long, b: Long, c: Long, d: Long, nohold: Long): Long
    fun One_implicit_bounds(explicitHold: Long, implicitHold: Long, nohold: Long): Long
    fun One_implicit_bounds_deep(explicit: Long, implicit1: Long, implicit2: Long, nohold: Long): Long
}

class One internal constructor (
    internal val handle: Long,

    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any>,
    ) {

    internal class OneCleaner(val handle: Long, val lib: OneLib) : Runnable {
        override fun run() {
            lib.One_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OneLib> = OneLib::class.java
        internal val lib: OneLib = Native.load("somelib", libClass)
        fun transitivity(hold: One, nohold: One): One {
            
            val returnVal = lib.One_transitivity(hold.handle, nohold.handle);
        
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any> = listOf(hold)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
        fun cycle(hold: Two, nohold: One): One {
            
            val returnVal = lib.One_cycle(hold.handle, nohold.handle);
        
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any> = listOf(hold)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
        fun manyDependents(a: One, b: One, c: Two, d: Two, nohold: Two): One {
            
            val returnVal = lib.One_many_dependents(a.handle, b.handle, c.handle, d.handle, nohold.handle);
        
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any> = listOf(a, b, c, d)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
        fun returnOutlivesParam(hold: Two, nohold: One): One {
            
            val returnVal = lib.One_return_outlives_param(hold.handle, nohold.handle);
        
            val selfEdges: List<Any> = listOf()
            val longEdges: List<Any> = listOf(hold)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, longEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
        fun diamondTop(top: One, left: One, right: One, bottom: One): One {
            
            val returnVal = lib.One_diamond_top(top.handle, left.handle, right.handle, bottom.handle);
        
            val selfEdges: List<Any> = listOf()
            val topEdges: List<Any> = listOf(top, left, right, bottom)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, topEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
        fun diamondLeft(top: One, left: One, right: One, bottom: One): One {
            
            val returnVal = lib.One_diamond_left(top.handle, left.handle, right.handle, bottom.handle);
        
            val selfEdges: List<Any> = listOf()
            val leftEdges: List<Any> = listOf(left, bottom)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, leftEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
        fun diamondRight(top: One, left: One, right: One, bottom: One): One {
            
            val returnVal = lib.One_diamond_right(top.handle, left.handle, right.handle, bottom.handle);
        
            val selfEdges: List<Any> = listOf()
            val rightEdges: List<Any> = listOf(right, bottom)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, rightEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
        fun diamondBottom(top: One, left: One, right: One, bottom: One): One {
            
            val returnVal = lib.One_diamond_bottom(top.handle, left.handle, right.handle, bottom.handle);
        
            val selfEdges: List<Any> = listOf()
            val bottomEdges: List<Any> = listOf(bottom)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, bottomEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
        fun diamondAndNestedTypes(a: One, b: One, c: One, d: One, nohold: One): One {
            
            val returnVal = lib.One_diamond_and_nested_types(a.handle, b.handle, c.handle, d.handle, nohold.handle);
        
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any> = listOf(a, b, c, d)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
        fun implicitBounds(explicitHold: One, implicitHold: One, nohold: One): One {
            
            val returnVal = lib.One_implicit_bounds(explicitHold.handle, implicitHold.handle, nohold.handle);
        
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any> = listOf(explicitHold, implicitHold)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
        fun implicitBoundsDeep(explicit: One, implicit1: One, implicit2: One, nohold: One): One {
            
            val returnVal = lib.One_implicit_bounds_deep(explicit.handle, implicit1.handle, implicit2.handle, nohold.handle);
        
            val selfEdges: List<Any> = listOf()
            val aEdges: List<Any> = listOf(explicit, implicit1, implicit2)
            val handle = returnVal
            val returnOpaque = One(handle, selfEdges, aEdges)
            CLEANER.register(returnOpaque, One.OneCleaner(handle, One.lib));
            
            return returnOpaque
        
        }
    }

}
