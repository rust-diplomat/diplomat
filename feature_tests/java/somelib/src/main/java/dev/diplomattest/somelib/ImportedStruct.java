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

public class ImportedStruct {
    UnimportedEnum foo;
    byte count;
    

    List<Object> selfEdges = List.of();
    

    private ImportedStruct() {
    }

    ImportedStruct(MemorySegment structSegment) {
        this.selfEdges = selfEdges;
        

        var fooNative = dev.diplomattest.somelib.ntv.ImportedStruct.foo(structSegment);
        var fooVal = UnimportedEnum.fromInt(fooNative);
        this.foo = fooVal;
        var countNative = dev.diplomattest.somelib.ntv.ImportedStruct.count(structSegment);
        var countVal = countNative;
        this.count = countVal;
        

    }

    MemorySegment toNative(SegmentAllocator arena) {
        var returnVal = dev.diplomattest.somelib.ntv.ImportedStruct.allocate(arena);
        
        var fooNative = foo.toInt();
        dev.diplomattest.somelib.ntv.ImportedStruct.foo(returnVal, fooNative);
        var countNative = count;
        dev.diplomattest.somelib.ntv.ImportedStruct.count(returnVal, countNative);
        

        return returnVal;

    }
    
}

