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

public class MyStruct {
    byte a;
    boolean b;
    byte c;
    long d;
    int e;
    int f;
    MyEnum g;
    

    MemorySegment internal;
    SegmentAllocator arena;
    List<Object> selfEdges = List.of();
    

    private MyStruct(SegmentAllocator arena) {
        this.arena = arena;
    }

    MyStruct(SegmentAllocator arena, MemorySegment structSegment) {
        this.arena = arena;
        this.selfEdges = selfEdges;
        

        this.a = dev.diplomattest.somelib.ntv.MyStruct.a(structSegment);
        this.b = dev.diplomattest.somelib.ntv.MyStruct.b(structSegment);
        this.c = dev.diplomattest.somelib.ntv.MyStruct.c(structSegment);
        this.d = dev.diplomattest.somelib.ntv.MyStruct.d(structSegment);
        this.e = dev.diplomattest.somelib.ntv.MyStruct.e(structSegment);
        this.f = dev.diplomattest.somelib.ntv.MyStruct.f(structSegment);
        this.g = MyEnum.fromInt(dev.diplomattest.somelib.ntv.MyStruct.g(structSegment));
        

    }
    
    public static MyStruct new_() {
        
        var returnArena = (SegmentAllocator) Arena.ofAuto();
        var nativeVal = somelib_h.MyStruct_new(returnArena);
        
        
        var returnVal = new MyStruct(returnArena, nativeVal);
        return returnVal;
    }
    
}

