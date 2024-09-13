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

public class Bar {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;
    SegmentAllocator arena;

    List<Object> selfEdges = List.of();
    List<Object> bEdges = List.of();
    List<Object> aEdges = List.of();
    

    static class BarCleaner implements Runnable {

        MemorySegment segment;
        BarCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.Bar_destroy(this.segment);
        }
    }

    Bar() {}
    Bar(MemorySegment handle, List<Object> selfEdges, List<Object> bEdges, List<Object> aEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        this.bEdges = bEdges;
        this.aEdges = aEdges;
        

    }
    
    
    public Foo foo() {
        
        
        var nativeVal = somelib_h.Bar_foo(internal);
        
        List<Object> selfEdges = List.of(this);
        
        
        
        List<Object> aEdges = List.of(this);
        var returnVal = new Foo(nativeVal, selfEdges, aEdges);
        return returnVal;
                
    }
    
}