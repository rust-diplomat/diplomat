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

public class CyclicStructA {
    CyclicStructB a;
    

    MemorySegment internal;
    SegmentAllocator arena;
    List<Object> selfEdges = List.of();
    

    private CyclicStructA(SegmentAllocator arena) {
        this.arena = arena;
    }

    CyclicStructA(SegmentAllocator arena, MemorySegment structSegment) {
        this.arena = arena;
        this.selfEdges = selfEdges;
        

        this.a = new CyclicStructB(arena, dev.diplomattest.somelib.ntv.CyclicStructA.a(structSegment));
        

    }
    
    public static CyclicStructB getB() {
        
        var returnArena = (SegmentAllocator) Arena.ofAuto();
        var nativeVal = somelib_h.CyclicStructA_get_b(returnArena);
        
        
        var returnVal = new CyclicStructB(returnArena, nativeVal);
        return returnVal;
    }
    
}

