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

public class RefList {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;
    SegmentAllocator arena;

    List<Object> selfEdges = List.of();
    List<Object> aEdges = List.of();
    

    static class RefListCleaner implements Runnable {

        MemorySegment segment;
        RefListCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.RefList_destroy(this.segment);
        }
    }

    RefList() {}
    RefList(MemorySegment handle, List<Object> selfEdges, List<Object> aEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        this.aEdges = aEdges;
        

    }
    
    public static RefList node(RefListParameter data) {
        
        var dataNative = data.internal;
        var nativeVal = somelib_h.RefList_node(dataNative);
        
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> bEdges = List.of(data);
        var returnVal = new RefList(nativeVal, selfEdges, bEdges);
        return returnVal;
                
    }
    
    
}