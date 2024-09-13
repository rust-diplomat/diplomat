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

public class MyZst {
    

    List<Object> selfEdges = List.of();
    

    private MyZst() {
    }

    MyZst(MemorySegment structSegment) {
        this.selfEdges = selfEdges;
        

        

    }

    
}

