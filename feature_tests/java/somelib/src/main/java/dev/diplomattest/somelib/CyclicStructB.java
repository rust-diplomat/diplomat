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
    

    List<Object> selfEdges = List.of();
    

    private CyclicStructB() {
    }

    CyclicStructB(MemorySegment structSegment) {
        this.selfEdges = selfEdges;
        

        var fieldNative = dev.diplomattest.somelib.ntv.CyclicStructB.field(structSegment);
        var fieldVal = fieldNative;
        this.field = fieldVal;
        

    }

    MemorySegment toNative(SegmentAllocator arena) {
        var returnVal = dev.diplomattest.somelib.ntv.CyclicStructB.allocate(arena);
        
        var fieldNative = field;
        dev.diplomattest.somelib.ntv.CyclicStructB.field(returnVal, fieldNative);
        

        return returnVal;

    }
    
    public static CyclicStructA getA() {
        
        try (var arena = Arena.ofConfined()) {
            
            var nativeVal = somelib_h.CyclicStructB_get_a(arena);
            
            var returnVal = new CyclicStructA(nativeVal);
            return returnVal;
                    
        }
    }
    
}

