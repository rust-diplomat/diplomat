package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.somelib_h;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import java.lang.foreign.SegmentAllocator;
import java.util.List;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;
import java.util.stream.Stream;

public class ImportedStruct {
    UnimportedEnum foo;
    byte count;
    

    MemorySegment internal;
    SegmentAllocator arena;
    List<Object> selfEdges = List.of();
    

    private ImportedStruct(SegmentAllocator arena) {
        this.arena = arena;
    }

    ImportedStruct(SegmentAllocator arena, MemorySegment structSegment) {
        this.arena = arena;
        this.selfEdges = selfEdges;
        

        this.foo = UnimportedEnum.fromInt(dev.diplomattest.somelib.ntv.ImportedStruct.foo(structSegment));
        this.count = dev.diplomattest.somelib.ntv.ImportedStruct.count(structSegment);
        

    }
    
}

