package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.*;


import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SegmentAllocator;
import java.lang.ref.Cleaner;
import java.util.List;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;
import java.util.stream.Stream;

public class Foo {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;
    SegmentAllocator arena;

    List<Object> selfEdges = List.of();
    List<Object> aEdges = List.of();
    

    static class FooCleaner implements Runnable {

        MemorySegment segment;
        FooCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.Foo_destroy(this.segment);
        }
    }

    Foo() {}
    Foo(MemorySegment handle, List<Object> selfEdges, List<Object> aEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        this.aEdges = aEdges;
        

    }
    
    public static Foo new_(String x) {
        
        try (var arena = Arena.ofConfined()) {
            var xData= Arena.ofAuto().allocateFrom(x, StandardCharsets.UTF_8);
            var xLen = xData.byteSize() - 1;  // allocated strings are null terminated
            var xView = DiplomatStringView.allocate(Arena.ofAuto());
            DiplomatStringView.len(xView, xLen);
            DiplomatStringView.data(xView, xData);
            var nativeVal = somelib_h.Foo_new(xView);
            
            List<Object> selfEdges = List.of();
            
            
            
            List<Object> aEdges = List.of(x);
            var returnVal = new Foo(nativeVal, selfEdges, aEdges);
            return returnVal;
                    
        }
    }
    
    public static Foo extractFromFields(BorrowedFields fields) {
        
        try (var arena = Arena.ofConfined()) {
            var fieldsNative = fields.toNative(arena);
            var nativeVal = somelib_h.Foo_extract_from_fields(fieldsNative);
            
            List<Object> selfEdges = List.of();
            
            
            
            List<Object> aEdges = List.of(fields);
            var returnVal = new Foo(nativeVal, selfEdges, aEdges);
            return returnVal;
                    
        }
    }
    
    public static Foo extractFromBounds(BorrowedFieldsWithBounds bounds,String anotherString) {
        
        try (var arena = Arena.ofConfined()) {
            var boundsNative = bounds.toNative(arena);
            var anotherStringData= Arena.ofAuto().allocateFrom(anotherString, StandardCharsets.UTF_8);
            var anotherStringLen = anotherStringData.byteSize() - 1;  // allocated strings are null terminated
            var anotherStringView = DiplomatStringView.allocate(Arena.ofAuto());
            DiplomatStringView.len(anotherStringView, anotherStringLen);
            DiplomatStringView.data(anotherStringView, anotherStringData);
            var nativeVal = somelib_h.Foo_extract_from_bounds(boundsNative, anotherStringView);
            
            List<Object> selfEdges = List.of();
            
            
            
            List<Object> aEdges = List.of(bounds, bounds, anotherString);
            var returnVal = new Foo(nativeVal, selfEdges, aEdges);
            return returnVal;
                    
        }
    }
    
    
    public Bar getBar() {
        
        
        var nativeVal = somelib_h.Foo_get_bar(internal);
        
        List<Object> selfEdges = List.of();
        
        
        
        List<Object> bEdges = List.of(this);
        List<Object> aEdges = List.of(this);
        var returnVal = new Bar(nativeVal, selfEdges, bEdges, aEdges);
        return returnVal;
                
    }
    
    public BorrowedFieldsReturning asReturning() {
        
        try (var arena = Arena.ofConfined()) {
            
            
            var nativeVal = somelib_h.Foo_as_returning(arena, internal);
            
            var returnVal = new BorrowedFieldsReturning(nativeVal, List.of());
            return returnVal;
                    
        }
    }
    
}