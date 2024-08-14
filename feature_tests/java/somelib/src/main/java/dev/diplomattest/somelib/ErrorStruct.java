package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.*;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import java.lang.foreign.SegmentAllocator;
import java.util.List;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;
import java.util.stream.Stream;

public class ErrorStruct {
    int i;
    int j;
    

    MemorySegment internal;
    SegmentAllocator arena;
    List<Object> selfEdges = List.of();
    

    private ErrorStruct(SegmentAllocator arena) {
        this.arena = arena;
    }

    ErrorStruct(SegmentAllocator arena, MemorySegment structSegment) {
        this.arena = arena;
        this.selfEdges = selfEdges;
        

        this.i = dev.diplomattest.somelib.ntv.ErrorStruct.i(structSegment);
        this.j = dev.diplomattest.somelib.ntv.ErrorStruct.j(structSegment);
        

    }
    
}

