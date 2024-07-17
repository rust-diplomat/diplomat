package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.somelib_h;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SegmentAllocator;
import java.lang.ref.Cleaner;
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

    SegmentAllocator ownArena;

    MemorySegment internal;

    private MyStruct(SegmentAllocator arena) {
        ownArena = arena;
    }

    void initFromSegment(MemorySegment segment) {
        internal = segment;
        a = dev.diplomattest.somelib.ntv.MyStruct.a(segment);
        b = dev.diplomattest.somelib.ntv.MyStruct.b(segment);
        c = dev.diplomattest.somelib.ntv.MyStruct.c(segment);
        d = dev.diplomattest.somelib.ntv.MyStruct.d(segment);
        e = dev.diplomattest.somelib.ntv.MyStruct.e(segment);
        f = dev.diplomattest.somelib.ntv.MyStruct.f(segment);
        g = MyEnum.fromInt(dev.diplomattest.somelib.ntv.MyStruct.g(segment));
    }

    public static MyStruct new_() {
        var arena = Arena.ofAuto();
        var nativeInvoker = somelib_h.MyStruct_new.makeInvoker();
        var nativeVal = nativeInvoker.apply(arena);
        var returnVal = new MyStruct(arena);
        returnVal.initFromSegment(nativeVal);
        return returnVal;
    }

}
