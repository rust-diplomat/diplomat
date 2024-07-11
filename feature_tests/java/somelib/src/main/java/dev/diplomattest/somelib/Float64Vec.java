package dev.diplomattest.somelib;

import java.lang.foreign.AddressLayout;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;

import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import java.nio.charset.StandardCharsets;

import com.sun.jna.Memory;
import com.sun.jna.Native;
import com.sun.jna.Pointer;

import dev.diplomattest.somelib.ntv.somelib_h;

import static java.lang.foreign.ValueLayout.JAVA_DOUBLE;


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

    Float64Vec() {
    }

    public double[] asSLice() {
        try (var arena = Arena.ofConfined()) {
            var nativeVal = somelib_h.Float64Vec_as_slice(arena, internal);
            var data = dev.diplomattest.somelib.ntv.DiplomatF64View.data(nativeVal);
            var len = dev.diplomattest.somelib.ntv.DiplomatF64View.len(nativeVal);
            return data.asSlice(0, len * JAVA_DOUBLE.byteSize()).toArray(JAVA_DOUBLE);
        }
    }

    public static Float64Vec new_(double[] v) {
        try (var arena = Arena.ofConfined()) {
            var vLen = v.length;
            var vMemSeg = arena.allocate(JAVA_DOUBLE, vLen);
            for (var i = 0; i < vLen; i++) {
                vMemSeg.setAtIndex(JAVA_DOUBLE, i, v[i]);
            }
            var nativeVal = somelib_h.Float64Vec_new(vMemSeg, vLen);
            var returnVal = new Float64Vec();
            returnVal.internal = nativeVal;
            var cleaner = new Float64Vec.Float64VecCleaner(nativeVal);
            returnVal.cleanable = Main.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }
}