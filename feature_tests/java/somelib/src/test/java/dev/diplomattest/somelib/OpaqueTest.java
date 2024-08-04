package dev.diplomattest.somelib;

import org.junit.jupiter.api.Test;

import java.lang.foreign.Arena;
import java.lang.foreign.SegmentAllocator;

import static org.junit.jupiter.api.Assertions.*;

class OpaqueTest {
    @Test
    void testOpaque() {
        var input = "How do you do?";
        var opaque = Opaque.fromStr(input);
        var debugStr = opaque.getDebugStr();
        assertEquals(input.length(), opaque.internalLen());
        assertEquals("\"How do you do?\"", debugStr);
    }
}