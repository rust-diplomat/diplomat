package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.*;


import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SegmentAllocator;
import java.lang.ref.Cleaner;
import java.util.List;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;
import java.util.stream.Stream;

public class One {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;

    List<Object> selfEdges = List.of();
    List<Object> aEdges = List.of();
    

    static class OneCleaner implements Runnable {

        MemorySegment segment;
        OneCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.One_destroy(this.segment);
        }
    }

    One() {}
    One(MemorySegment handle, List<Object> selfEdges, List<Object> aEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        this.aEdges = aEdges;
        

    }
    
    public static One transitivity(One hold,One nohold) {
        
        var holdNative = hold.internal;
        var noholdNative = nohold.internal;
        var nativeVal = somelib_h.One_transitivity(holdNative, noholdNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> aEdges = List.of(hold);
        var returnVal = new One(nativeVal, selfEdges, aEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static One cycle(Two hold,One nohold) {
        
        var holdNative = hold.internal;
        var noholdNative = nohold.internal;
        var nativeVal = somelib_h.One_cycle(holdNative, noholdNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> aEdges = List.of(hold);
        var returnVal = new One(nativeVal, selfEdges, aEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static One manyDependents(One a,One b,Two c,Two d,Two nohold) {
        
        var aNative = a.internal;
        var bNative = b.internal;
        var cNative = c.internal;
        var dNative = d.internal;
        var noholdNative = nohold.internal;
        var nativeVal = somelib_h.One_many_dependents(aNative, bNative, cNative, dNative, noholdNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> aEdges = List.of(a, b, c, d);
        var returnVal = new One(nativeVal, selfEdges, aEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static One returnOutlivesParam(Two hold,One nohold) {
        
        var holdNative = hold.internal;
        var noholdNative = nohold.internal;
        var nativeVal = somelib_h.One_return_outlives_param(holdNative, noholdNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> longEdges = List.of(hold);
        var returnVal = new One(nativeVal, selfEdges, longEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static One diamondTop(One top,One left,One right,One bottom) {
        
        var topNative = top.internal;
        var leftNative = left.internal;
        var rightNative = right.internal;
        var bottomNative = bottom.internal;
        var nativeVal = somelib_h.One_diamond_top(topNative, leftNative, rightNative, bottomNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> topEdges = List.of(top, left, right, bottom);
        var returnVal = new One(nativeVal, selfEdges, topEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static One diamondLeft(One top,One left,One right,One bottom) {
        
        var topNative = top.internal;
        var leftNative = left.internal;
        var rightNative = right.internal;
        var bottomNative = bottom.internal;
        var nativeVal = somelib_h.One_diamond_left(topNative, leftNative, rightNative, bottomNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> leftEdges = List.of(left, bottom);
        var returnVal = new One(nativeVal, selfEdges, leftEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static One diamondRight(One top,One left,One right,One bottom) {
        
        var topNative = top.internal;
        var leftNative = left.internal;
        var rightNative = right.internal;
        var bottomNative = bottom.internal;
        var nativeVal = somelib_h.One_diamond_right(topNative, leftNative, rightNative, bottomNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> rightEdges = List.of(right, bottom);
        var returnVal = new One(nativeVal, selfEdges, rightEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static One diamondBottom(One top,One left,One right,One bottom) {
        
        var topNative = top.internal;
        var leftNative = left.internal;
        var rightNative = right.internal;
        var bottomNative = bottom.internal;
        var nativeVal = somelib_h.One_diamond_bottom(topNative, leftNative, rightNative, bottomNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> bottomEdges = List.of(bottom);
        var returnVal = new One(nativeVal, selfEdges, bottomEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static One diamondAndNestedTypes(One a,One b,One c,One d,One nohold) {
        
        var aNative = a.internal;
        var bNative = b.internal;
        var cNative = c.internal;
        var dNative = d.internal;
        var noholdNative = nohold.internal;
        var nativeVal = somelib_h.One_diamond_and_nested_types(aNative, bNative, cNative, dNative, noholdNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> aEdges = List.of(a, b, c, d);
        var returnVal = new One(nativeVal, selfEdges, aEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static One implicitBounds(One explicitHold,One implicitHold,One nohold) {
        
        var explicitHoldNative = explicitHold.internal;
        var implicitHoldNative = implicitHold.internal;
        var noholdNative = nohold.internal;
        var nativeVal = somelib_h.One_implicit_bounds(explicitHoldNative, implicitHoldNative, noholdNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> aEdges = List.of(explicitHold, implicitHold);
        var returnVal = new One(nativeVal, selfEdges, aEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static One implicitBoundsDeep(One explicit,One implicit1,One implicit2,One nohold) {
        
        var explicitNative = explicit.internal;
        var implicit1Native = implicit1.internal;
        var implicit2Native = implicit2.internal;
        var noholdNative = nohold.internal;
        var nativeVal = somelib_h.One_implicit_bounds_deep(explicitNative, implicit1Native, implicit2Native, noholdNative);
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> aEdges = List.of(explicit, implicit1, implicit2);
        var returnVal = new One(nativeVal, selfEdges, aEdges);
        var cleaner = new One.OneCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    
}