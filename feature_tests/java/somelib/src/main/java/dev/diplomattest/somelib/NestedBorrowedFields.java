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
    

    List<Object> selfEdges = List.of();
    List<Object> xEdges = List.of();
    List<Object> yEdges = List.of();
    List<Object> zEdges = List.of();
    

    private NestedBorrowedFields() {
    }

    NestedBorrowedFields(MemorySegment structSegment, List<Object> xEdges, List<Object> yEdges, List<Object> zEdges) {
        this.selfEdges = selfEdges;
        this.xEdges = xEdges;
        this.yEdges = yEdges;
        this.zEdges = zEdges;
        

        var fieldsNative = dev.diplomattest.somelib.ntv.NestedBorrowedFields.fields(structSegment);
        var fieldsVal = new BorrowedFields(fieldsNative, List.of(xEdges, yEdges));
        this.fields = fieldsVal;
        var boundsNative = dev.diplomattest.somelib.ntv.NestedBorrowedFields.bounds(structSegment);
        var boundsVal = new BorrowedFieldsWithBounds(boundsNative, List.of(xEdges, yEdges), List.of(), List.of());
        this.bounds = boundsVal;
        var bounds2Native = dev.diplomattest.somelib.ntv.NestedBorrowedFields.bounds2(structSegment);
        var bounds2Val = new BorrowedFieldsWithBounds(bounds2Native, List.of(), List.of(), List.of());
        this.bounds2 = bounds2Val;
        

    }

    MemorySegment toNative(SegmentAllocator arena) {
        var returnVal = dev.diplomattest.somelib.ntv.NestedBorrowedFields.allocate(arena);
        
        var fieldsNative = fields.toNative(arena);
        dev.diplomattest.somelib.ntv.NestedBorrowedFields.fields(returnVal, fieldsNative);
        var boundsNative = bounds.toNative(arena);
        dev.diplomattest.somelib.ntv.NestedBorrowedFields.bounds(returnVal, boundsNative);
        var bounds2Native = bounds2.toNative(arena);
        dev.diplomattest.somelib.ntv.NestedBorrowedFields.bounds2(returnVal, bounds2Native);
        

        return returnVal;

    }
    
    public static NestedBorrowedFields fromBarAndFooAndStrings(Bar bar,Foo foo,String dstr16X,String dstr16Z,String utf8StrY,String utf8StrZ) {
        
        try (var arena = Arena.ofConfined()) {
            
            var barNative = bar.internal;
            var fooNative = foo.internal;
            var dstr16XData = Arena.ofAuto().allocateFrom(dstr16X, StandardCharsets.UTF_16);
            var dstr16XLen = dstr16XData.byteSize() - 1;  // allocated strings are null terminated
            var dstr16XView = DiplomatString16View.allocate(Arena.ofAuto());
            DiplomatString16View.len(dstr16XView, dstr16XLen);
            DiplomatString16View.data(dstr16XView, dstr16XData);
            var dstr16ZData = Arena.ofAuto().allocateFrom(dstr16Z, StandardCharsets.UTF_16);
            var dstr16ZLen = dstr16ZData.byteSize() - 1;  // allocated strings are null terminated
            var dstr16ZView = DiplomatString16View.allocate(Arena.ofAuto());
            DiplomatString16View.len(dstr16ZView, dstr16ZLen);
            DiplomatString16View.data(dstr16ZView, dstr16ZData);
            var utf8StrYData= Arena.ofAuto().allocateFrom(utf8StrY, StandardCharsets.UTF_8);
            var utf8StrYLen = utf8StrYData.byteSize() - 1;  // allocated strings are null terminated
            var utf8StrYView = DiplomatStringView.allocate(Arena.ofAuto());
            DiplomatStringView.len(utf8StrYView, utf8StrYLen);
            DiplomatStringView.data(utf8StrYView, utf8StrYData);
            var utf8StrZData= Arena.ofAuto().allocateFrom(utf8StrZ, StandardCharsets.UTF_8);
            var utf8StrZLen = utf8StrZData.byteSize() - 1;  // allocated strings are null terminated
            var utf8StrZView = DiplomatStringView.allocate(Arena.ofAuto());
            DiplomatStringView.len(utf8StrZView, utf8StrZLen);
            DiplomatStringView.data(utf8StrZView, utf8StrZData);
            var nativeVal = somelib_h.NestedBorrowedFields_from_bar_and_foo_and_strings(arena, barNative, fooNative, dstr16XView, dstr16ZView, utf8StrYView, utf8StrZView);
            
            var returnVal = new NestedBorrowedFields(nativeVal, List.of(bar, dstr16X, utf8StrY), List.of(bar, utf8StrY), List.of(foo, dstr16Z, utf8StrZ));
            return returnVal;
                    
        }
    }
    
}

