package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.somelib_h;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import java.lang.foreign.SegmentAllocator;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;

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

    private MyStruct(SegmentAllocator arena) {
    }

    void initFromSegment(MemorySegment segment) {
        this.internal = segment;

        this.a = dev.diplomattest.somelib.ntv.MyStruct.a(segment);
        this.b = dev.diplomattest.somelib.ntv.MyStruct.b(segment);
        this.c = dev.diplomattest.somelib.ntv.MyStruct.c(segment);
        this.d = dev.diplomattest.somelib.ntv.MyStruct.d(segment);
        this.e = dev.diplomattest.somelib.ntv.MyStruct.e(segment);
        this.f = dev.diplomattest.somelib.ntv.MyStruct.f(segment);
        this.g = MyEnum.fromInt(dev.diplomattest.somelib.ntv.MyStruct.g(segment));

    }

    public static MyStruct new_() {

        var returnArena = (SegmentAllocator) Arena.ofAuto();
        var nativeInvoker = somelib_h.MyStruct_new.makeInvoker();
        var nativeVal = nativeInvoker.apply(returnArena);
        var returnVal = new MyStruct(returnArena);
        returnVal.initFromSegment(nativeVal);
        return returnVal;
    }

}

