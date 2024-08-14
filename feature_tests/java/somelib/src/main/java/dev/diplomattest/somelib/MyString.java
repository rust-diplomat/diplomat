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

public class MyString {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;

    List<Object> selfEdges = List.of();
    

    static class MyStringCleaner implements Runnable {

        MemorySegment segment;
        MyStringCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.MyString_destroy(this.segment);
        }
    }

    MyString() {}
    MyString(MemorySegment handle, List<Object> selfEdges) {
        this.internal = handle;
        this.selfEdges = selfEdges;
        

    }
    
    public static MyString new_(String v) {
        
        try (var arena = Arena.ofConfined()) {
            var vData= arena.allocateFrom(v, StandardCharsets.UTF_8);
            var vLen = vData.byteSize() - 1;
            var vView = DiplomatStringView.allocate(arena);
            DiplomatStringView.len(vView, vLen);
            DiplomatStringView.data(vView, vData);
            var nativeVal = somelib_h.MyString_new(vView);
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new MyString(nativeVal, selfEdges);
            var cleaner = new MyString.MyStringCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }
    
    public static MyString newUnsafe(String v) {
        
        try (var arena = Arena.ofConfined()) {
            var vData= arena.allocateFrom(v, StandardCharsets.UTF_8);
            var vLen = vData.byteSize() - 1;
            var vView = DiplomatStringView.allocate(arena);
            DiplomatStringView.len(vView, vLen);
            DiplomatStringView.data(vView, vData);
            var nativeVal = somelib_h.MyString_new_unsafe(vView);
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new MyString(nativeVal, selfEdges);
            var cleaner = new MyString.MyStringCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }
    
    public static MyString newOwned(String v) {
        
        var vData= Arena.global().allocateFrom(v, StandardCharsets.UTF_8);
        var vLen = vData.byteSize() - 1;
        var vView = DiplomatStringView.allocate(Arena.global());
        DiplomatStringView.len(vView, vLen);
        DiplomatStringView.data(vView, vData);
        var nativeVal = somelib_h.MyString_new_owned(vView);
        List<Object> selfEdges = List.of();
        
        
        
        var returnVal = new MyString(nativeVal, selfEdges);
        var cleaner = new MyString.MyStringCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }
    
    public static MyString newFromFirst(String [] v) {
        
        try (var arena = Arena.ofConfined()) {
            var vView = SliceUtils.strs8(arena, v);
            var vLen = v.length;
            var nativeVal = somelib_h.MyString_new_from_first(vView);
            List<Object> selfEdges = List.of();
            
            
            
            var returnVal = new MyString(nativeVal, selfEdges);
            var cleaner = new MyString.MyStringCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }
    
    public static String stringTransform(String foo) {
        
        try (var arena = Arena.ofConfined()) {
            var fooData= arena.allocateFrom(foo, StandardCharsets.UTF_8);
            var fooLen = fooData.byteSize() - 1;
            var fooView = DiplomatStringView.allocate(arena);
            DiplomatStringView.len(fooView, fooLen);
            DiplomatStringView.data(fooView, fooData);
            var writeable = somelib_h.diplomat_buffer_write_create(0);
            somelib_h.MyString_string_transform(fooView, writeable);
            var string = SliceUtils. readUtf8FromWriteable(writeable);
            somelib_h.diplomat_buffer_write_destroy(writeable);
            return string;
        }
    }
    
    
    public void setStr(String newStr) {
        
        try (var arena = Arena.ofConfined()) {
            
            var newStrData= arena.allocateFrom(newStr, StandardCharsets.UTF_8);
            var newStrLen = newStrData.byteSize() - 1;
            var newStrView = DiplomatStringView.allocate(arena);
            DiplomatStringView.len(newStrView, newStrLen);
            DiplomatStringView.data(newStrView, newStrData);
            somelib_h.MyString_set_str(internal, newStrView);
            
        }
    }
    
    public String getStr() {
        
        
        var writeable = somelib_h.diplomat_buffer_write_create(0);
        somelib_h.MyString_get_str(internal, writeable);
        var string = SliceUtils. readUtf8FromWriteable(writeable);
        somelib_h.diplomat_buffer_write_destroy(writeable);
        return string;
    }
    
}