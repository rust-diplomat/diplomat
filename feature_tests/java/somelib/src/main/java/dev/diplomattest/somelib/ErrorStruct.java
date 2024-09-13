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
    

    List<Object> selfEdges = List.of();
    

    private ErrorStruct() {
    }

    ErrorStruct(MemorySegment structSegment) {
        this.selfEdges = selfEdges;
        

        var iNative = dev.diplomattest.somelib.ntv.ErrorStruct.i(structSegment);
        var iVal = iNative;
        this.i = iVal;
        var jNative = dev.diplomattest.somelib.ntv.ErrorStruct.j(structSegment);
        var jVal = jNative;
        this.j = jVal;
        

    }

    MemorySegment toNative(SegmentAllocator arena) {
        var returnVal = dev.diplomattest.somelib.ntv.ErrorStruct.allocate(arena);
        
        var iNative = i;
        dev.diplomattest.somelib.ntv.ErrorStruct.i(returnVal, iNative);
        var jNative = j;
        dev.diplomattest.somelib.ntv.ErrorStruct.j(returnVal, jNative);
        

        return returnVal;

    }
    
}

