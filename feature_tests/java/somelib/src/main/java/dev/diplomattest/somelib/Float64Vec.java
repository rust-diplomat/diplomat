package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.*;


import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;

public class Float64Vec {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;

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

    public static Float64Vec new_(double[] v) {
        try (var arena = Arena.ofConfined()) {
            var vLen = v.length;
            var vMemSeg = arena.allocateFrom(JAVA_DOUBLE, v);
            var nativeVal = somelib_h.Float64Vec_new(vMemSeg, vLen);
            var returnVal = new Float64Vec();
            returnVal.internal = nativeVal;
            var cleaner = new Float64Vec.Float64VecCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }


    public double[] asBoxedSlice() {
        var boxArena = Arena.ofConfined();

        var nativeVal = somelib_h.Float64Vec_as_boxed_slice(boxArena, internal);
        var data = dev.diplomattest.somelib.ntv.DiplomatF64View.data(nativeVal);
        var len = dev.diplomattest.somelib.ntv.DiplomatF64View.len(nativeVal);
        return SliceUtils.doubleSliceToArray(nativeVal);
    }

    public double[] asSlice() {
        try (var arena = Arena.ofConfined()) {


            var nativeVal = somelib_h.Float64Vec_as_slice(arena, internal);
            var data = dev.diplomattest.somelib.ntv.DiplomatF64View.data(nativeVal);
            var len = dev.diplomattest.somelib.ntv.DiplomatF64View.len(nativeVal);
            return SliceUtils.doubleSliceToArray(nativeVal);
        }
    }

}
