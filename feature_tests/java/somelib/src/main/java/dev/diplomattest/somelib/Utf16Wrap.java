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

public class Utf16Wrap {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;
    SegmentAllocator arena;

    List<Object> selfEdges = List.of();
    

    static class Utf16WrapCleaner implements Runnable {

        MemorySegment segment;
        Utf16WrapCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.Utf16Wrap_destroy(this.segment);
        }
    }

    Utf16Wrap() {}
    Utf16Wrap(MemorySegment handle, List<Object> selfEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        

    }
    
    public static Utf16Wrap fromUtf16(String input) {
        
        try (var arena = Arena.ofConfined()) {
            var inputData = Arena.ofAuto().allocateFrom(input, StandardCharsets.UTF_16);
            var inputLen = inputData.byteSize() - 1;  // allocated strings are null terminated
            var inputView = DiplomatString16View.allocate(Arena.ofAuto());
            DiplomatString16View.len(inputView, inputLen);
            DiplomatString16View.data(inputView, inputData);
            var nativeVal = somelib_h.Utf16Wrap_from_utf16(inputView);
            
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new Utf16Wrap(nativeVal, selfEdges);
            return returnVal;
                    
        }
    }
    
    
    public String getDebugStr() {
        
        
        var writeable = somelib_h.diplomat_buffer_write_create(0);
        somelib_h.Utf16Wrap_get_debug_str(internal, writeable);
        var string = SliceUtils. readUtf8FromWriteable(writeable);
        somelib_h.diplomat_buffer_write_destroy(writeable);
        return string;
    }
    
    public String borrowCont() {
        
        try (var arena = Arena.ofConfined()) {
            
            
            var nativeVal = somelib_h.Utf16Wrap_borrow_cont(arena, internal);
            
            var returnVal = SliceUtils.readUtf16(nativeVal);
            return returnVal;
                    
        }
    }
    
}