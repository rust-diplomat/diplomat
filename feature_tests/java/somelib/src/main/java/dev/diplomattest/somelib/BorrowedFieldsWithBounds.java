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
            var dstr16XMemSeg = arena.allocateFrom(dstr16X, StandardCharsets.UTF_16);
            var dstr16XLen = dstr16XMemSeg.byteSize();
            var utf8StrZMemSeg = arena.allocateFrom(utf8StrZ, StandardCharsets.UTF_8);
            var utf8StrZLen = utf8StrZMemSeg.byteSize();
            var nativeVal = somelib_h.BorrowedFieldsWithBounds_from_foo_and_strings(returnArena, fooNative, dstr16XMemSeg, dstr16XLen - 1, utf8StrZMemSeg, utf8StrZLen - 1);
            
            List<Object> xEdges = List.of(foo, dstr16X, utf8StrZ);
            List<Object> yEdges = List.of(foo, utf8StrZ);
            List<Object> zEdges = List.of(utf8StrZ);
            
            var returnVal = new BorrowedFieldsWithBounds(returnArena, nativeVal, xEdges, yEdges, zEdges);
            return returnVal;
        }
    }
    
}

