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

public class Float64Vec {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;

    List<Object> selfEdges = List.of();
    

    static class Float64VecCleaner implements Runnable {

        MemorySegment segment;
        Float64VecCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.Float64Vec_destroy(this.segment);
        }
    }

    Float64Vec() {}
    Float64Vec(MemorySegment handle, List<Object> selfEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        

    }
    
    public static Float64Vec newBool(boolean[] v) {
        
        try (var arena = Arena.ofConfined()) {
            var vLen = v.length;
            byte[] vByteArray = new byte[vLen];
            for (int i = 0; i < vLen; i++) {
                vByteArray[i] = (byte) (v[i] ? 1 : 0);
            }
            var vData = arena.allocateFrom(JAVA_BYTE, vByteArray);
            var vView = DiplomatBoolView.allocate(arena);
            DiplomatBoolView.len(vView, vLen);
            DiplomatBoolView.data(vView, vData);
    
            var nativeVal = somelib_h.Float64Vec_new_bool(vView);
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new Float64Vec(nativeVal, selfEdges);
            var cleaner = new Float64Vec.Float64VecCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }
    
    public static Float64Vec newI16(short[] v) {
        
        try (var arena = Arena.ofConfined()) {
            var vLen = v.length;
            var vData= arena.allocateFrom(JAVA_SHORT, v);
            var vView = DiplomatI16View.allocate(arena);
            DiplomatI16View.len(vView, vLen);
            DiplomatI16View.data(vView, vData);
    
            var nativeVal = somelib_h.Float64Vec_new_i16(vView);
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new Float64Vec(nativeVal, selfEdges);
            var cleaner = new Float64Vec.Float64VecCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }
    
    public static Float64Vec newU16(short[] v) {
        
        try (var arena = Arena.ofConfined()) {
            var vLen = v.length;
            var vData= arena.allocateFrom(JAVA_SHORT, v);
            var vView = DiplomatU16View.allocate(arena);
            DiplomatU16View.len(vView, vLen);
            DiplomatU16View.data(vView, vData);
    
            var nativeVal = somelib_h.Float64Vec_new_u16(vView);
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new Float64Vec(nativeVal, selfEdges);
            var cleaner = new Float64Vec.Float64VecCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }
    
    public static Float64Vec newIsize(long[] v) {
        
        try (var arena = Arena.ofConfined()) {
            var vLen = v.length;
            var vData= arena.allocateFrom(JAVA_LONG, v);
            var vView = DiplomatIsizeView.allocate(arena);
            DiplomatIsizeView.len(vView, vLen);
            DiplomatIsizeView.data(vView, vData);
    
            var nativeVal = somelib_h.Float64Vec_new_isize(vView);
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new Float64Vec(nativeVal, selfEdges);
            var cleaner = new Float64Vec.Float64VecCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }
    
    public static Float64Vec newUsize(long[] v) {
        
        try (var arena = Arena.ofConfined()) {
            var vLen = v.length;
            var vData= arena.allocateFrom(JAVA_LONG, v);
            var vView = DiplomatUsizeView.allocate(arena);
            DiplomatUsizeView.len(vView, vLen);
            DiplomatUsizeView.data(vView, vData);
    
            var nativeVal = somelib_h.Float64Vec_new_usize(vView);
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new Float64Vec(nativeVal, selfEdges);
            var cleaner = new Float64Vec.Float64VecCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }
    
    public static Float64Vec newF64BeBytes(byte[] v) {
        
        try (var arena = Arena.ofConfined()) {
            var vLen = v.length;
            var vData= arena.allocateFrom(JAVA_BYTE, v);
            var vView = DiplomatU8View.allocate(arena);
            DiplomatU8View.len(vView, vLen);
            DiplomatU8View.data(vView, vData);
    
            var nativeVal = somelib_h.Float64Vec_new_f64_be_bytes(vView);
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new Float64Vec(nativeVal, selfEdges);
            var cleaner = new Float64Vec.Float64VecCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }
    
    public static Float64Vec newFromOwned(double[] v) {
        
        var vLen = v.length;
        var vData= Arena.global().allocateFrom(JAVA_DOUBLE, v);
        var vView = DiplomatF64View.allocate(Arena.global());
        DiplomatF64View.len(vView, vLen);
        DiplomatF64View.data(vView, vData);
    
        var nativeVal = somelib_h.Float64Vec_new_from_owned(vView);
        List<Object> selfEdges = List.of();
        
        
        
        var returnVal = new Float64Vec(nativeVal, selfEdges);
        var cleaner = new Float64Vec.Float64VecCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    
    public double[] asSlice() {
        
        try (var arena = Arena.ofConfined()) {
            
            
            var nativeVal = somelib_h.Float64Vec_as_slice(arena, internal);
            var data = dev.diplomattest.somelib.ntv.DiplomatF64View.data(nativeVal);
            var len = dev.diplomattest.somelib.ntv.DiplomatF64View.len(nativeVal);
            return SliceUtils.doubleSliceToArray(nativeVal);
        }
    }
    
    public void fillSlice(double[] v) {
        
        try (var arena = Arena.ofConfined()) {
            
            var vLen = v.length;
            var vData= arena.allocateFrom(JAVA_DOUBLE, v);
            var vView = DiplomatF64View.allocate(arena);
            DiplomatF64View.len(vView, vLen);
            DiplomatF64View.data(vView, vData);
    
            somelib_h.Float64Vec_fill_slice(internal, vView);
            
        }
    }
    
    public void setValue(double[] newSlice) {
        
        try (var arena = Arena.ofConfined()) {
            
            var newSliceLen = newSlice.length;
            var newSliceData= arena.allocateFrom(JAVA_DOUBLE, newSlice);
            var newSliceView = DiplomatF64View.allocate(arena);
            DiplomatF64View.len(newSliceView, newSliceLen);
            DiplomatF64View.data(newSliceView, newSliceData);
    
            somelib_h.Float64Vec_set_value(internal, newSliceView);
            
        }
    }
    
    public String toString_() {
        
        
        var writeable = somelib_h.diplomat_buffer_write_create(0);
        somelib_h.Float64Vec_to_string(internal, writeable);
        var string = SliceUtils. readUtf8FromWriteable(writeable);
        somelib_h.diplomat_buffer_write_destroy(writeable);
        return string;
    }
    
    public double[] borrow() {
        
        try (var arena = Arena.ofConfined()) {
            
            
            var nativeVal = somelib_h.Float64Vec_borrow(arena, internal);
            var data = dev.diplomattest.somelib.ntv.DiplomatF64View.data(nativeVal);
            var len = dev.diplomattest.somelib.ntv.DiplomatF64View.len(nativeVal);
            return SliceUtils.doubleSliceToArray(nativeVal);
        }
    }
    
}