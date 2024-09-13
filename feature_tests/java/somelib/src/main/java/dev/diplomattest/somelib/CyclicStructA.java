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

public class CyclicStructA {
    CyclicStructB a;
    

    List<Object> selfEdges = List.of();
    

    private CyclicStructA() {
    }

    CyclicStructA(MemorySegment structSegment) {
        this.selfEdges = selfEdges;
        

        var aNative = dev.diplomattest.somelib.ntv.CyclicStructA.a(structSegment);
        var aVal = new CyclicStructB(aNative);
        this.a = aVal;
        

    }

    MemorySegment toNative(SegmentAllocator arena) {
        var returnVal = dev.diplomattest.somelib.ntv.CyclicStructA.allocate(arena);
        
        var aNative = a.toNative(arena);
        dev.diplomattest.somelib.ntv.CyclicStructA.a(returnVal, aNative);
        

        return returnVal;

    }
    
    public static CyclicStructB getB() {
        
        try (var arena = Arena.ofConfined()) {
            
            var nativeVal = somelib_h.CyclicStructA_get_b(arena);
            
            var returnVal = new CyclicStructB(nativeVal);
            return returnVal;
                    
        }
    }
    
}

