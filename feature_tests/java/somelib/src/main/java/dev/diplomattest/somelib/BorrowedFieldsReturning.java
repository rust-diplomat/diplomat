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
    

    List<Object> selfEdges = List.of();
    List<Object> aEdges = List.of();
    

    private BorrowedFieldsReturning() {
    }

    BorrowedFieldsReturning(MemorySegment structSegment, List<Object> aEdges) {
        this.selfEdges = selfEdges;
        this.aEdges = aEdges;
        

        var bytesNative = dev.diplomattest.somelib.ntv.BorrowedFieldsReturning.bytes(structSegment);
        var bytesVal = SliceUtils.readUtf8(bytesNative);
        this.bytes = bytesVal;
        

    }

    MemorySegment toNative(SegmentAllocator arena) {
        var returnVal = dev.diplomattest.somelib.ntv.BorrowedFieldsReturning.allocate(arena);
        
        var bytesData= arena.allocateFrom(bytes, StandardCharsets.UTF_8);
        var bytesLen = bytesData.byteSize() - 1;  // allocated strings are null terminated
        var bytesView = DiplomatStringView.allocate(arena);
        DiplomatStringView.len(bytesView, bytesLen);
        DiplomatStringView.data(bytesView, bytesData);
        dev.diplomattest.somelib.ntv.BorrowedFieldsReturning.bytes(returnVal, bytesView);
        

        return returnVal;

    }
    
}

