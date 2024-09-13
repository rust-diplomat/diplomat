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
    

    List<Object> selfEdges = List.of();
    List<Object> aEdges = List.of();
    

    private BorrowedFields() {
    }

    BorrowedFields(MemorySegment structSegment, List<Object> aEdges) {
        this.selfEdges = selfEdges;
        this.aEdges = aEdges;
        

        var aNative = dev.diplomattest.somelib.ntv.BorrowedFields.a(structSegment);
        var aVal = SliceUtils.readUtf16(aNative);
        this.a = aVal;
        var bNative = dev.diplomattest.somelib.ntv.BorrowedFields.b(structSegment);
        var bVal = SliceUtils.readUtf8(bNative);
        this.b = bVal;
        var cNative = dev.diplomattest.somelib.ntv.BorrowedFields.c(structSegment);
        var cVal = SliceUtils.readUtf8(cNative);
        this.c = cVal;
        

    }

    MemorySegment toNative(SegmentAllocator arena) {
        var returnVal = dev.diplomattest.somelib.ntv.BorrowedFields.allocate(arena);
        
        var aData = arena.allocateFrom(a, StandardCharsets.UTF_16);
        var aLen = aData.byteSize() - 1;  // allocated strings are null terminated
        var aView = DiplomatString16View.allocate(arena);
        DiplomatString16View.len(aView, aLen);
        DiplomatString16View.data(aView, aData);
        dev.diplomattest.somelib.ntv.BorrowedFields.a(returnVal, aView);
        var bData= arena.allocateFrom(b, StandardCharsets.UTF_8);
        var bLen = bData.byteSize() - 1;  // allocated strings are null terminated
        var bView = DiplomatStringView.allocate(arena);
        DiplomatStringView.len(bView, bLen);
        DiplomatStringView.data(bView, bData);
        dev.diplomattest.somelib.ntv.BorrowedFields.b(returnVal, bView);
        var cData= arena.allocateFrom(c, StandardCharsets.UTF_8);
        var cLen = cData.byteSize() - 1;  // allocated strings are null terminated
        var cView = DiplomatStringView.allocate(arena);
        DiplomatStringView.len(cView, cLen);
        DiplomatStringView.data(cView, cData);
        dev.diplomattest.somelib.ntv.BorrowedFields.c(returnVal, cView);
        

        return returnVal;

    }
    
    public static BorrowedFields fromBarAndStrings(Bar bar,String dstr16,String utf8Str) {
        
        try (var arena = Arena.ofConfined()) {
            
            var barNative = bar.internal;
            var dstr16Data = Arena.ofAuto().allocateFrom(dstr16, StandardCharsets.UTF_16);
            var dstr16Len = dstr16Data.byteSize() - 1;  // allocated strings are null terminated
            var dstr16View = DiplomatString16View.allocate(Arena.ofAuto());
            DiplomatString16View.len(dstr16View, dstr16Len);
            DiplomatString16View.data(dstr16View, dstr16Data);
            var utf8StrData= Arena.ofAuto().allocateFrom(utf8Str, StandardCharsets.UTF_8);
            var utf8StrLen = utf8StrData.byteSize() - 1;  // allocated strings are null terminated
            var utf8StrView = DiplomatStringView.allocate(Arena.ofAuto());
            DiplomatStringView.len(utf8StrView, utf8StrLen);
            DiplomatStringView.data(utf8StrView, utf8StrData);
            var nativeVal = somelib_h.BorrowedFields_from_bar_and_strings(arena, barNative, dstr16View, utf8StrView);
            
            var returnVal = new BorrowedFields(nativeVal, List.of(bar, dstr16, utf8Str));
            return returnVal;
                    
        }
    }
    
}

