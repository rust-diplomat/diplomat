package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.somelib_h;
import dev.diplomattest.somelib.ntv.somelib_h;

import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;

public class Opaque {
    private MemorySegment internal;
    private final Cleaner.Cleanable cleanable;
    static class OpaqueCleaner implements Runnable {
        MemorySegment segment;
        OpaqueCleaner(MemorySegment segment) {
            this.segment = segment;
        }
        public void run() {
            somelib_h.Opaque_destroy(this.segment);
        }
    }
    public Opaque() {
        var invoker = somelib_h.Opaque_new.makeInvoker();
        var segment = invoker.apply();
        this.internal = segment;
        var opaqueCleaner = new OpaqueCleaner(segment);
        this.cleanable = Main.cleaner.register(this, opaqueCleaner);
    }

    public long pointer() {
        return internal.address();
    }

    public void delete() {
        somelib_h.Opaque_destroy(this.internal);
    }

    public void assertStruct(MyStruct struct) {
        somelib_h.Opaque_assert_struct(internal, struct.getNativeStruct$());
    }
}
