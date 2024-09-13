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

public class MyStruct {
    byte a;
    boolean b;
    byte c;
    long d;
    int e;
    int f;
    MyEnum g;
    

    List<Object> selfEdges = List.of();
    

    private MyStruct() {
    }

    MyStruct(MemorySegment structSegment) {
        this.selfEdges = selfEdges;
        

        var aNative = dev.diplomattest.somelib.ntv.MyStruct.a(structSegment);
        var aVal = aNative;
        this.a = aVal;
        var bNative = dev.diplomattest.somelib.ntv.MyStruct.b(structSegment);
        var bVal = bNative;
        this.b = bVal;
        var cNative = dev.diplomattest.somelib.ntv.MyStruct.c(structSegment);
        var cVal = cNative;
        this.c = cVal;
        var dNative = dev.diplomattest.somelib.ntv.MyStruct.d(structSegment);
        var dVal = dNative;
        this.d = dVal;
        var eNative = dev.diplomattest.somelib.ntv.MyStruct.e(structSegment);
        var eVal = eNative;
        this.e = eVal;
        var fNative = dev.diplomattest.somelib.ntv.MyStruct.f(structSegment);
        var fVal = fNative;
        this.f = fVal;
        var gNative = dev.diplomattest.somelib.ntv.MyStruct.g(structSegment);
        var gVal = MyEnum.fromInt(gNative);
        this.g = gVal;
        

    }

    MemorySegment toNative(SegmentAllocator arena) {
        var returnVal = dev.diplomattest.somelib.ntv.MyStruct.allocate(arena);
        
        var aNative = a;
        dev.diplomattest.somelib.ntv.MyStruct.a(returnVal, aNative);
        var bNative = b;
        dev.diplomattest.somelib.ntv.MyStruct.b(returnVal, bNative);
        var cNative = c;
        dev.diplomattest.somelib.ntv.MyStruct.c(returnVal, cNative);
        var dNative = d;
        dev.diplomattest.somelib.ntv.MyStruct.d(returnVal, dNative);
        var eNative = e;
        dev.diplomattest.somelib.ntv.MyStruct.e(returnVal, eNative);
        var fNative = f;
        dev.diplomattest.somelib.ntv.MyStruct.f(returnVal, fNative);
        var gNative = g.toInt();
        dev.diplomattest.somelib.ntv.MyStruct.g(returnVal, gNative);
        

        return returnVal;

    }
    
    public static MyStruct new_() {
        
        try (var arena = Arena.ofConfined()) {
            
            var nativeVal = somelib_h.MyStruct_new(arena);
            
            var returnVal = new MyStruct(nativeVal);
            return returnVal;
                    
        }
    }
    
}

