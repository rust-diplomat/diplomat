package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.somelib_h;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SegmentAllocator;

public class MyStruct {
    public byte a;
    public boolean b;
    public byte c;
    public long d;
    public int e;
    public char f;
    public MyEnum g;

    MemorySegment internal = null;

    MyStruct(SegmentAllocator alloc, byte a, boolean b, byte c, long d, int e, char f, MyEnum g) {
        var segment = dev.diplomattest.somelib.ntv.MyStruct.allocate(alloc);
        dev.diplomattest.somelib.ntv.MyStruct.a(segment, a);
        dev.diplomattest.somelib.ntv.MyStruct.b(segment, b);
        dev.diplomattest.somelib.ntv.MyStruct.c(segment, c);
        dev.diplomattest.somelib.ntv.MyStruct.d(segment, d);
        dev.diplomattest.somelib.ntv.MyStruct.e(segment, e);
        dev.diplomattest.somelib.ntv.MyStruct.f(segment, f);
        dev.diplomattest.somelib.ntv.MyStruct.g(segment, g.toInt());
        internal = segment;
    }
    private MyStruct() {}

    static MyStruct fromSegment(MemorySegment segment) {
        var returnVal = new MyStruct();
        returnVal.internal = segment;
        returnVal.a = dev.diplomattest.somelib.ntv.MyStruct.a(segment);
        returnVal.b = dev.diplomattest.somelib.ntv.MyStruct.b(segment);
        returnVal.c = dev.diplomattest.somelib.ntv.MyStruct.c(segment);
        returnVal.d = dev.diplomattest.somelib.ntv.MyStruct.d(segment);
        returnVal.e = dev.diplomattest.somelib.ntv.MyStruct.e(segment);
        returnVal.f = (char) dev.diplomattest.somelib.ntv.MyStruct.f(segment);
        returnVal.g = MyEnum.fromInt(dev.diplomattest.somelib.ntv.MyStruct.g(segment));
        return returnVal;
    }

    public static MyStruct new_() {
        var function = somelib_h.MyStruct_new.makeInvoker();
        var arena = Arena.global();
        var nativeVal = function.apply(arena);
        return MyStruct.fromSegment(nativeVal);
    }

    MemorySegment getNativeStruct$() {
        return internal;
    }
}
