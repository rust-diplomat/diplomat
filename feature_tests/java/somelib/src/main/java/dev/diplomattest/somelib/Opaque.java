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

public class Opaque {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;
    SegmentAllocator arena;

    List<Object> selfEdges = List.of();
    

    static class OpaqueCleaner implements Runnable {

        MemorySegment segment;
        OpaqueCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.Opaque_destroy(this.segment);
        }
    }

    Opaque() {}
    Opaque(MemorySegment handle, List<Object> selfEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        

    }
    
    public static Opaque new_() {
        
        var nativeVal = somelib_h.Opaque_new();
        
        List<Object> selfEdges = List.of();
        
        
        
        var returnVal = new Opaque(nativeVal, selfEdges);
        return returnVal;
                
    }
    
    public static Opaque fromStr(String input) {
        
        try (var arena = Arena.ofConfined()) {
            var inputData= Arena.ofAuto().allocateFrom(input, StandardCharsets.UTF_8);
            var inputLen = inputData.byteSize() - 1;  // allocated strings are null terminated
            var inputView = DiplomatStringView.allocate(Arena.ofAuto());
            DiplomatStringView.len(inputView, inputLen);
            DiplomatStringView.data(inputView, inputData);
            var nativeVal = somelib_h.Opaque_from_str(inputView);
            
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new Opaque(nativeVal, selfEdges);
            return returnVal;
                    
        }
    }
    
    public static long returnsUsize() {
        
        var nativeVal = somelib_h.Opaque_returns_usize();
        
        var returnVal = nativeVal;
        return returnVal;
                
    }
    
    public static ImportedStruct returnsImported() {
        
        try (var arena = Arena.ofConfined()) {
            
            var nativeVal = somelib_h.Opaque_returns_imported(arena);
            
            var returnVal = new ImportedStruct(nativeVal);
            return returnVal;
                    
        }
    }
    
    public static byte cmp() {
        
        var nativeVal = somelib_h.Opaque_cmp();
        
        var returnVal = nativeVal;
        return returnVal;
                
    }
    
    
    public String getDebugStr() {
        
        
        var writeable = somelib_h.diplomat_buffer_write_create(0);
        somelib_h.Opaque_get_debug_str(internal, writeable);
        var string = SliceUtils. readUtf8FromWriteable(writeable);
        somelib_h.diplomat_buffer_write_destroy(writeable);
        return string;
    }
    
    public void assertStruct(MyStruct s) {
        
        
        var sNative = s.toNative(Arena.ofAuto());
        somelib_h.Opaque_assert_struct(internal, sNative);
        
    }
    
    public long internalLen() {
        
        
        var nativeVal = somelib_h.Opaque_internal_len(internal);
        
        var returnVal = nativeVal;
        return returnVal;
                
    }
    
}