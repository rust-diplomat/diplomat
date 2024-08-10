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

public class OpaqueMutexedString {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;

    List<Object> selfEdges = List.of();
    

    static class OpaqueMutexedStringCleaner implements Runnable {

        MemorySegment segment;
        OpaqueMutexedStringCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.OpaqueMutexedString_destroy(this.segment);
        }
    }

    OpaqueMutexedString() {}
    OpaqueMutexedString(MemorySegment handle, List<Object> selfEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        

    }
    
    public static OpaqueMutexedString fromUsize(long number) {
        
        var numberNative = number;
        var nativeVal = somelib_h.OpaqueMutexedString_from_usize(numberNative);
        List<Object> selfEdges = List.of();
        
        
        
        var returnVal = new OpaqueMutexedString(nativeVal, selfEdges);
        var cleaner = new OpaqueMutexedString.OpaqueMutexedStringCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static OpaqueMutexedString borrowOther(OpaqueMutexedString other) {
        
        var otherNative = other.internal;
        var nativeVal = somelib_h.OpaqueMutexedString_borrow_other(otherNative);
        List<Object> selfEdges = List.of(other);
        
        
        
        var returnVal = new OpaqueMutexedString(nativeVal, selfEdges);
        
        return returnVal;
    }
    
    
    public void change(long number) {
        
        
        var numberNative = number;
        somelib_h.OpaqueMutexedString_change(internal, numberNative);
        
    }
    
    public OpaqueMutexedString borrow() {
        
        
        var nativeVal = somelib_h.OpaqueMutexedString_borrow(internal);
        List<Object> selfEdges = List.of(this);
        
        
        
        var returnVal = new OpaqueMutexedString(nativeVal, selfEdges);
        
        return returnVal;
    }
    
    public OpaqueMutexedString borrowSelfOrOther(OpaqueMutexedString other) {
        
        
        var otherNative = other.internal;
        var nativeVal = somelib_h.OpaqueMutexedString_borrow_self_or_other(internal, otherNative);
        List<Object> selfEdges = List.of(this, other);
        
        
        
        var returnVal = new OpaqueMutexedString(nativeVal, selfEdges);
        
        return returnVal;
    }
    
    public long getLenAndAdd(long other) {
        
        
        var otherNative = other;
        var nativeVal = somelib_h.OpaqueMutexedString_get_len_and_add(internal, otherNative);
        return nativeVal;
    }
    
    public String dummyStr() {
        
        try (var arena = Arena.ofConfined()) {
            
            
            var nativeVal = somelib_h.OpaqueMutexedString_dummy_str(arena, internal);
            return SliceUtils.readUtf8(nativeVal);
        }
    }
    
    public Utf16Wrap wrapper() {
        
        
        var nativeVal = somelib_h.OpaqueMutexedString_wrapper(internal);
        List<Object> selfEdges = List.of();
        
        
        
        var returnVal = new Utf16Wrap(nativeVal, selfEdges);
        var cleaner = new Utf16Wrap.Utf16WrapCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
}