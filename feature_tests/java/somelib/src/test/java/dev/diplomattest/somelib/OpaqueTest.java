package dev.diplomattest.somelib;

import org.junit.jupiter.api.Test;

import java.lang.foreign.Arena;
import java.lang.foreign.SegmentAllocator;

import static org.junit.jupiter.api.Assertions.*;

class OpaqueTest {
    @Test
    void testOpaque() {
        var opaque = new Opaque();

        assert opaque.pointer() != 0L;

        try (var arena = Arena.ofConfined()) {
            var myStruct = new MyStruct(arena);
            opaque.assertStruct(myStruct);
        }

        opaque.delete();
        assert true;
    }
}