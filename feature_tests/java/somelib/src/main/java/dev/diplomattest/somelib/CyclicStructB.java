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

public class CyclicStructB {
    byte field;
    

    SegmentAllocator arena;
    List<Object> selfEdges = List.of();
    

    private CyclicStructB(SegmentAllocator arena) {
        this.arena = arena;
    }

    CyclicStructB(SegmentAllocator arena, MemorySegment structSegment) {
        this.arena = arena;
        this.selfEdges = selfEdges;
        

        this.field = dev.diplomattest.somelib.ntv.CyclicStructB.field(structSegment);
        

    }

    MemorySegment toNative(SegmentAllocator arena) {
        var returnVal = dev.diplomattest.somelib.ntv.CyclicStructB.allocate(arena);
        
        dev.diplomattest.somelib.ntv.CyclicStructB.field(returnVal, this.field);
        

        return returnVal;

    }
    
    public static CyclicStructA getA() {
        
        var returnArena = (SegmentAllocator) Arena.ofAuto();
        var nativeVal = somelib_h.CyclicStructB_get_a(returnArena);
        
        
        var returnVal = new CyclicStructA(returnArena, nativeVal);
        return returnVal;
    }
    
}

