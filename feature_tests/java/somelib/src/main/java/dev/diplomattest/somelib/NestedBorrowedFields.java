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

public class NestedBorrowedFields {
    BorrowedFields fields;
    BorrowedFieldsWithBounds bounds;
    BorrowedFieldsWithBounds bounds2;
    

    MemorySegment internal;
    SegmentAllocator arena;
    List<Object> selfEdges = List.of();
    List<Object> xEdges = List.of();
    List<Object> yEdges = List.of();
    List<Object> zEdges = List.of();
    

    private NestedBorrowedFields(SegmentAllocator arena) {
        this.arena = arena;
    }

    NestedBorrowedFields(SegmentAllocator arena, MemorySegment structSegment, List<Object> xEdges, List<Object> yEdges, List<Object> zEdges) {
        this.arena = arena;
        this.selfEdges = selfEdges;
        this.xEdges = xEdges;
        this.yEdges = yEdges;
        this.zEdges = zEdges;
        

        this.fields = new BorrowedFields(arena, dev.diplomattest.somelib.ntv.NestedBorrowedFields.fields(structSegment), Stream.concat(xEdges.stream(), yEdges.stream()).toList());
        this.bounds = new BorrowedFieldsWithBounds(arena, dev.diplomattest.somelib.ntv.NestedBorrowedFields.bounds(structSegment), Stream.concat(xEdges.stream(), yEdges.stream()).toList(), yEdges, yEdges);
        this.bounds2 = new BorrowedFieldsWithBounds(arena, dev.diplomattest.somelib.ntv.NestedBorrowedFields.bounds2(structSegment), zEdges, zEdges, zEdges);
        

    }
    
    public static NestedBorrowedFields fromBarAndFooAndStrings(Bar bar,Foo foo,String dstr16X,String dstr16Z,String utf8StrY,String utf8StrZ) {
        
        try (var arena = Arena.ofConfined()) {
            var returnArena = (SegmentAllocator) Arena.ofAuto();
            var barNative = bar.internal;
            var fooNative = foo.internal;
            var dstr16XMemSeg = arena.allocateFrom(dstr16X, StandardCharsets.UTF_16);
            var dstr16XLen = dstr16XMemSeg.byteSize();
            var dstr16ZMemSeg = arena.allocateFrom(dstr16Z, StandardCharsets.UTF_16);
            var dstr16ZLen = dstr16ZMemSeg.byteSize();
            var utf8StrYMemSeg = arena.allocateFrom(utf8StrY, StandardCharsets.UTF_8);
            var utf8StrYLen = utf8StrYMemSeg.byteSize();
            var utf8StrZMemSeg = arena.allocateFrom(utf8StrZ, StandardCharsets.UTF_8);
            var utf8StrZLen = utf8StrZMemSeg.byteSize();
            var nativeVal = somelib_h.NestedBorrowedFields_from_bar_and_foo_and_strings(returnArena, barNative, fooNative, dstr16XMemSeg, dstr16XLen - 1, dstr16ZMemSeg, dstr16ZLen - 1, utf8StrYMemSeg, utf8StrYLen - 1, utf8StrZMemSeg, utf8StrZLen - 1);
            
            List<Object> xEdges = List.of(bar, dstr16X, utf8StrY);
            List<Object> yEdges = List.of(bar, utf8StrY);
            List<Object> zEdges = List.of(foo, dstr16Z, utf8StrZ);
            
            var returnVal = new NestedBorrowedFields(returnArena, nativeVal, xEdges, yEdges, zEdges);
            return returnVal;
        }
    }
    
}

