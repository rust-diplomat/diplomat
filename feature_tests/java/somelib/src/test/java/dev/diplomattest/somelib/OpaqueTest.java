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
        System.out.println("instantiated");
        System.out.println(opaque.getDebugStr());
        System.out.println("done debuging");
        assertEquals(input.length(), opaque.internalLen());
    }
}