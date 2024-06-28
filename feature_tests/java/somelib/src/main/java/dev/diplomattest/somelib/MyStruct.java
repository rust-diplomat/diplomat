package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.somelib_h;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.SegmentAllocator;

public class MyStruct {
    MemorySegment nativeStruct$;
    public MyStruct(SegmentAllocator alloc) {
        var function = somelib_h.MyStruct_new.makeInvoker();
        this.nativeStruct$ = function.apply(alloc);
    }

    MemorySegment getNativeStruct$() {
        return nativeStruct$;
    }
}
