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

public class BorrowedFieldsReturning {
    String bytes;
    

    MemorySegment internal;
    SegmentAllocator arena;
    List<Object> selfEdges = List.of();
    List<Object> aEdges = List.of();
    

    private BorrowedFieldsReturning(SegmentAllocator arena) {
        this.arena = arena;
    }

    BorrowedFieldsReturning(SegmentAllocator arena, MemorySegment structSegment, List<Object> aEdges) {
        this.arena = arena;
        this.selfEdges = selfEdges;
        this.aEdges = aEdges;
        

        this.bytes = SliceUtils.readUtf8(dev.diplomattest.somelib.ntv.BorrowedFieldsReturning.bytes(structSegment));
        

    }
    
}

