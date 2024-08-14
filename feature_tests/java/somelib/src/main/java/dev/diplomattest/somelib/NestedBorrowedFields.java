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
            var dstr16XData= arena.allocateFrom(dstr16X, StandardCharsets.UTF_16);
            var dstr16XLen = dstr16XData.byteSize() - 1;  // allocated strings are null terminated
            var dstr16XView = DiplomatString16View.allocate(arena);
            DiplomatString16View.len(dstr16XView, dstr16XLen);
            DiplomatString16View.data(dstr16XView, dstr16XData);
            var dstr16ZData= arena.allocateFrom(dstr16Z, StandardCharsets.UTF_16);
            var dstr16ZLen = dstr16ZData.byteSize() - 1;  // allocated strings are null terminated
            var dstr16ZView = DiplomatString16View.allocate(arena);
            DiplomatString16View.len(dstr16ZView, dstr16ZLen);
            DiplomatString16View.data(dstr16ZView, dstr16ZData);
            var utf8StrYData= arena.allocateFrom(utf8StrY, StandardCharsets.UTF_8);
            var utf8StrYLen = utf8StrYData.byteSize() - 1;
            var utf8StrYView = DiplomatStringView.allocate(arena);
            DiplomatStringView.len(utf8StrYView, utf8StrYLen);
            DiplomatStringView.data(utf8StrYView, utf8StrYData);
            var utf8StrZData= arena.allocateFrom(utf8StrZ, StandardCharsets.UTF_8);
            var utf8StrZLen = utf8StrZData.byteSize() - 1;
            var utf8StrZView = DiplomatStringView.allocate(arena);
            DiplomatStringView.len(utf8StrZView, utf8StrZLen);
            DiplomatStringView.data(utf8StrZView, utf8StrZData);
            var nativeVal = somelib_h.NestedBorrowedFields_from_bar_and_foo_and_strings(returnArena, barNative, fooNative, dstr16XView, dstr16ZView, utf8StrYView, utf8StrZView);
            
            List<Object> xEdges = List.of(bar, dstr16X, utf8StrY);
            List<Object> yEdges = List.of(bar, utf8StrY);
            List<Object> zEdges = List.of(foo, dstr16Z, utf8StrZ);
            
            var returnVal = new NestedBorrowedFields(returnArena, nativeVal, xEdges, yEdges, zEdges);
            return returnVal;
        }
    }
    
}

