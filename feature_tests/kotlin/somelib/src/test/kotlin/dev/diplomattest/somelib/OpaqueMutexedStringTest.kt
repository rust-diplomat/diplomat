package dev.diplomattest.somelib

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class OpaqueMutexedStringTest {
    @Test
    fun testLoad() {
        val opaqueMutexedString = OpaqueMutexedString.fromUsize(356.toULong())
        assertEquals(opaqueMutexedString.getLenAndAdd(4.toULong()), 7.toULong())
        val borrowed = opaqueMutexedString.borrow()
        borrowed.change(1234.toULong())
        assertEquals(opaqueMutexedString.getLenAndAdd(4.toULong()), 8.toULong())
        Result
    }

    @Test
    fun testMultiBorrow() {
        val opaqueMutexedStringOddLen = OpaqueMutexedString.fromUsize(356.toULong())
        val opaqueMutexedStringEvenLen = OpaqueMutexedString.fromUsize(1111.toULong())
        val borrowed = opaqueMutexedStringEvenLen.borrowSelfOrOther(opaqueMutexedStringOddLen)
        assertEquals(borrowed.getLenAndAdd(0.toULong()), 4.toULong())

        opaqueMutexedStringEvenLen.change(33333.toULong())
        val borrowed2 = opaqueMutexedStringEvenLen.borrowSelfOrOther(opaqueMutexedStringOddLen)
        assertEquals(borrowed2.getLenAndAdd(0.toULong()), 3.toULong())
    }

    @Test
    fun testStrReturn() {
        val opaque = OpaqueMutexedString.fromUsize(356.toULong())
        val str = opaque.dummyStr()

        assertEquals("A const str with non byte char: 餐 which is a DiplomatChar,", str)

        val wrapper = opaque.wrapper()
        val newStr = wrapper.borrowCont()
        val testStr = "A const str with non byte char: 𐐷 which is a DiplomatChar,"
        assertEquals(testStr, newStr)

        for (it in 0..1_000_000) {
            wrapper.owned()
        }

        val ownedStr = wrapper.owned()
        assertEquals(testStr, ownedStr)
    }
}
