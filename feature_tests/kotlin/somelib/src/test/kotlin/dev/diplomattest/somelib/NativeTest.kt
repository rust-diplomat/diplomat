package dev.diplomattest.somelib

import org.junit.jupiter.api.Test
import java.lang.foreign.MemorySession
import java.lang.foreign.SegmentAllocator
import kotlin.random.Random
import kotlin.test.assertEquals

class NativeTest {
    @Test
    @ExperimentalUnsignedTypes
    fun testNative() {
        val random =  Random(2984751)
        for (run in 0 .. 100) {
            val array = random.nextBytes(1000)
            val (mem, slice) = PrimitiveArrayTools.native(array)
            val got = slice.data.getByteArray(0, array.size)
            mem.close()
            assertEquals(got.toList(), array.toList())
        }

        for (run in 0 .. 100) {
            val array = random.nextBytes(1000)
            val uByteArray = array.map { it.toUByte()}.toUByteArray()
            val (mem, slice) = PrimitiveArrayTools.native(array)
            val got = slice.data.getByteArray(0, array.size).asUByteArray()
            mem.close()
            assertEquals(got.map { it.toUByte()}, uByteArray.toList())
        }


        for (run in 0 .. 100) {
            val size = 1000
            val intArray = (0..size).map { random.nextInt() }.toIntArray()
            val (mem, slice) = PrimitiveArrayTools.native(intArray)
            val got = slice.data.getIntArray(0, intArray.size)
            mem.close()
            assertEquals(got.toList(), intArray.toList())
        }

        for (run in 0 .. 100) {
            val size = 1000
            val intArray = (0..size).map { random.nextInt().toUInt() }.toUIntArray()
            val (mem, slice) = PrimitiveArrayTools.native(intArray)
            val got = slice.data.getIntArray(0, intArray.size).asUIntArray()
            mem.close()
            assertEquals(got.toList(), intArray.toList())
        }

    }
}