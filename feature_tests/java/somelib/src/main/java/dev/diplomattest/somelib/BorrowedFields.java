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
            var dstr16MemSeg = arena.allocateFrom(dstr16, StandardCharsets.UTF_16);
            var dstr16Len = dstr16MemSeg.byteSize();
            var utf8StrMemSeg = arena.allocateFrom(utf8Str, StandardCharsets.UTF_8);
            var utf8StrLen = utf8StrMemSeg.byteSize();
            var nativeVal = somelib_h.BorrowedFields_from_bar_and_strings(returnArena, barNative, dstr16MemSeg, dstr16Len - 1, utf8StrMemSeg, utf8StrLen - 1);
            
            List<Object> xEdges = List.of(bar, dstr16, utf8Str);
            
            var returnVal = new BorrowedFields(returnArena, nativeVal, xEdges);
            return returnVal;
        }
    }
    
}

