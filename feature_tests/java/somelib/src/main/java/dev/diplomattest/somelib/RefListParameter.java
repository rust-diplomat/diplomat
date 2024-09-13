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

public class RefListParameter {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;
    SegmentAllocator arena;

    List<Object> selfEdges = List.of();
    

    static class RefListParameterCleaner implements Runnable {

        MemorySegment segment;
        RefListParameterCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.RefListParameter_destroy(this.segment);
        }
    }

    RefListParameter() {}
    RefListParameter(MemorySegment handle, List<Object> selfEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        

    }
    
    
}