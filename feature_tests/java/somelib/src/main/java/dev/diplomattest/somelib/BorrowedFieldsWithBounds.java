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

public class BorrowedFieldsWithBounds {
    String fieldA;
    String fieldB;
    String fieldC;
    

    MemorySegment internal;
    SegmentAllocator arena;
    List<Object> selfEdges = List.of();
    List<Object> aEdges = List.of();
    List<Object> bEdges = List.of();
    List<Object> cEdges = List.of();
    

    private BorrowedFieldsWithBounds(SegmentAllocator arena) {
        this.arena = arena;
    }

    BorrowedFieldsWithBounds(SegmentAllocator arena, MemorySegment structSegment, List<Object> aEdges, List<Object> bEdges, List<Object> cEdges) {
        this.arena = arena;
        this.selfEdges = selfEdges;
        this.aEdges = aEdges;
        this.bEdges = bEdges;
        this.cEdges = cEdges;
        

        this.fieldA = SliceUtils.readUtf16(dev.diplomattest.somelib.ntv.BorrowedFieldsWithBounds.field_a(structSegment));
        this.fieldB = SliceUtils.readUtf8(dev.diplomattest.somelib.ntv.BorrowedFieldsWithBounds.field_b(structSegment));
        this.fieldC = SliceUtils.readUtf8(dev.diplomattest.somelib.ntv.BorrowedFieldsWithBounds.field_c(structSegment));
        

    }
    
    public static BorrowedFieldsWithBounds fromFooAndStrings(Foo foo,String dstr16X,String utf8StrZ) {
        
        try (var arena = Arena.ofConfined()) {
            var returnArena = (SegmentAllocator) Arena.ofAuto();
            var fooNative = foo.internal;
            var dstr16XData= arena.allocateFrom(dstr16X, StandardCharsets.UTF_16);
            var dstr16XLen = dstr16XData.byteSize() - 1;  // allocated strings are null terminated
            var dstr16XView = DiplomatString16View.allocate(arena);
            DiplomatString16View.len(dstr16XView, dstr16XLen);
            DiplomatString16View.data(dstr16XView, dstr16XData);
            var utf8StrZData= arena.allocateFrom(utf8StrZ, StandardCharsets.UTF_8);
            var utf8StrZLen = utf8StrZData.byteSize() - 1;
            var utf8StrZView = DiplomatStringView.allocate(arena);
            DiplomatStringView.len(utf8StrZView, utf8StrZLen);
            DiplomatStringView.data(utf8StrZView, utf8StrZData);
            var nativeVal = somelib_h.BorrowedFieldsWithBounds_from_foo_and_strings(returnArena, fooNative, dstr16XView, utf8StrZView);
            
            List<Object> xEdges = List.of(foo, dstr16X, utf8StrZ);
            List<Object> yEdges = List.of(foo, utf8StrZ);
            List<Object> zEdges = List.of(utf8StrZ);
            
            var returnVal = new BorrowedFieldsWithBounds(returnArena, nativeVal, xEdges, yEdges, zEdges);
            return returnVal;
        }
    }
    
}

