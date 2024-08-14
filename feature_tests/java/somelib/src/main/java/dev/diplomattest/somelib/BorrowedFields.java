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

public class BorrowedFields {
    String a;
    String b;
    String c;
    

    MemorySegment internal;
    SegmentAllocator arena;
    List<Object> selfEdges = List.of();
    List<Object> aEdges = List.of();
    

    private BorrowedFields(SegmentAllocator arena) {
        this.arena = arena;
    }

    BorrowedFields(SegmentAllocator arena, MemorySegment structSegment, List<Object> aEdges) {
        this.arena = arena;
        this.selfEdges = selfEdges;
        this.aEdges = aEdges;
        

        this.a = SliceUtils.readUtf16(dev.diplomattest.somelib.ntv.BorrowedFields.a(structSegment));
        this.b = SliceUtils.readUtf8(dev.diplomattest.somelib.ntv.BorrowedFields.b(structSegment));
        this.c = SliceUtils.readUtf8(dev.diplomattest.somelib.ntv.BorrowedFields.c(structSegment));
        

    }
    
    public static BorrowedFields fromBarAndStrings(Bar bar,String dstr16,String utf8Str) {
        
        try (var arena = Arena.ofConfined()) {
            var returnArena = (SegmentAllocator) Arena.ofAuto();
            var barNative = bar.internal;
            var dstr16Data= arena.allocateFrom(dstr16, StandardCharsets.UTF_16);
            var dstr16Len = dstr16Data.byteSize() - 1;  // allocated strings are null terminated
            var dstr16View = DiplomatString16View.allocate(arena);
            DiplomatString16View.len(dstr16View, dstr16Len);
            DiplomatString16View.data(dstr16View, dstr16Data);
            var utf8StrData= arena.allocateFrom(utf8Str, StandardCharsets.UTF_8);
            var utf8StrLen = utf8StrData.byteSize() - 1;
            var utf8StrView = DiplomatStringView.allocate(arena);
            DiplomatStringView.len(utf8StrView, utf8StrLen);
            DiplomatStringView.data(utf8StrView, utf8StrData);
            var nativeVal = somelib_h.BorrowedFields_from_bar_and_strings(returnArena, barNative, dstr16View, utf8StrView);
            
            List<Object> xEdges = List.of(bar, dstr16, utf8Str);
            
            var returnVal = new BorrowedFields(returnArena, nativeVal, xEdges);
            return returnVal;
        }
    }
    
}

