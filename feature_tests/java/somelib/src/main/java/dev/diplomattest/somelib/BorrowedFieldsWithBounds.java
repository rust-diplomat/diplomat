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
    

    List<Object> selfEdges = List.of();
    List<Object> aEdges = List.of();
    List<Object> bEdges = List.of();
    List<Object> cEdges = List.of();
    

    private BorrowedFieldsWithBounds() {
    }

    BorrowedFieldsWithBounds(MemorySegment structSegment, List<Object> aEdges, List<Object> bEdges, List<Object> cEdges) {
        this.selfEdges = selfEdges;
        this.aEdges = aEdges;
        this.bEdges = bEdges;
        this.cEdges = cEdges;
        

        var fieldANative = dev.diplomattest.somelib.ntv.BorrowedFieldsWithBounds.field_a(structSegment);
        var fieldAVal = SliceUtils.readUtf16(fieldANative);
        this.fieldA = fieldAVal;
        var fieldBNative = dev.diplomattest.somelib.ntv.BorrowedFieldsWithBounds.field_b(structSegment);
        var fieldBVal = SliceUtils.readUtf8(fieldBNative);
        this.fieldB = fieldBVal;
        var fieldCNative = dev.diplomattest.somelib.ntv.BorrowedFieldsWithBounds.field_c(structSegment);
        var fieldCVal = SliceUtils.readUtf8(fieldCNative);
        this.fieldC = fieldCVal;
        

    }

    MemorySegment toNative(SegmentAllocator arena) {
        var returnVal = dev.diplomattest.somelib.ntv.BorrowedFieldsWithBounds.allocate(arena);
        
        var fieldAData = arena.allocateFrom(fieldA, StandardCharsets.UTF_16);
        var fieldALen = fieldAData.byteSize() - 1;  // allocated strings are null terminated
        var fieldAView = DiplomatString16View.allocate(arena);
        DiplomatString16View.len(fieldAView, fieldALen);
        DiplomatString16View.data(fieldAView, fieldAData);
        dev.diplomattest.somelib.ntv.BorrowedFieldsWithBounds.field_a(returnVal, fieldAView);
        var fieldBData= arena.allocateFrom(fieldB, StandardCharsets.UTF_8);
        var fieldBLen = fieldBData.byteSize() - 1;  // allocated strings are null terminated
        var fieldBView = DiplomatStringView.allocate(arena);
        DiplomatStringView.len(fieldBView, fieldBLen);
        DiplomatStringView.data(fieldBView, fieldBData);
        dev.diplomattest.somelib.ntv.BorrowedFieldsWithBounds.field_b(returnVal, fieldBView);
        var fieldCData= arena.allocateFrom(fieldC, StandardCharsets.UTF_8);
        var fieldCLen = fieldCData.byteSize() - 1;  // allocated strings are null terminated
        var fieldCView = DiplomatStringView.allocate(arena);
        DiplomatStringView.len(fieldCView, fieldCLen);
        DiplomatStringView.data(fieldCView, fieldCData);
        dev.diplomattest.somelib.ntv.BorrowedFieldsWithBounds.field_c(returnVal, fieldCView);
        

        return returnVal;

    }
    
    public static BorrowedFieldsWithBounds fromFooAndStrings(Foo foo,String dstr16X,String utf8StrZ) {
        
        try (var arena = Arena.ofConfined()) {
            
            var fooNative = foo.internal;
            var dstr16XData = Arena.ofAuto().allocateFrom(dstr16X, StandardCharsets.UTF_16);
            var dstr16XLen = dstr16XData.byteSize() - 1;  // allocated strings are null terminated
            var dstr16XView = DiplomatString16View.allocate(Arena.ofAuto());
            DiplomatString16View.len(dstr16XView, dstr16XLen);
            DiplomatString16View.data(dstr16XView, dstr16XData);
            var utf8StrZData= Arena.ofAuto().allocateFrom(utf8StrZ, StandardCharsets.UTF_8);
            var utf8StrZLen = utf8StrZData.byteSize() - 1;  // allocated strings are null terminated
            var utf8StrZView = DiplomatStringView.allocate(Arena.ofAuto());
            DiplomatStringView.len(utf8StrZView, utf8StrZLen);
            DiplomatStringView.data(utf8StrZView, utf8StrZData);
            var nativeVal = somelib_h.BorrowedFieldsWithBounds_from_foo_and_strings(arena, fooNative, dstr16XView, utf8StrZView);
            
            var returnVal = new BorrowedFieldsWithBounds(nativeVal, List.of(foo, dstr16X, utf8StrZ), List.of(foo, utf8StrZ), List.of());
            return returnVal;
                    
        }
    }
    
}

