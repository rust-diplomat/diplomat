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

public class Two {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;

    List<Object> selfEdges = List.of();
    List<Object> aEdges = List.of();
    List<Object> bEdges = List.of();
    

    static class TwoCleaner implements Runnable {

        MemorySegment segment;
        TwoCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.Two_destroy(this.segment);
        }
    }

    Two() {}
    Two(MemorySegment handle, List<Object> selfEdges, List<Object> aEdges, List<Object> bEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        this.aEdges = aEdges;
        this.bEdges = bEdges;
        

    }
    
    
}