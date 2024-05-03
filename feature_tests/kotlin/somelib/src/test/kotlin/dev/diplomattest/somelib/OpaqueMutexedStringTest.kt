package dev.diplomattest.somelib

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class OpaqueMutexedStringTest {
    @Test
    fun testLoad() {
        val opaqueMutexedString = OpaqueMutexedString.fromUsize(356)
        assertEquals(opaqueMutexedString.getLenAndAdd(4), 7)
        val borrowed = opaqueMutexedString.borrow()
        borrowed.change(1234)
        assertEquals(opaqueMutexedString.getLenAndAdd(4), 8)
        Result
    }

    @Test
    fun testMultiBorrow() {
        val opaqueMutexedStringOddLen = OpaqueMutexedString.fromUsize(356)
        val opaqueMutexedStringEvenLen = OpaqueMutexedString.fromUsize(1111)
        val borrowed = opaqueMutexedStringEvenLen.borrowSelfOrOther(opaqueMutexedStringOddLen)
        assertEquals(borrowed.getLenAndAdd(0), 4)

        opaqueMutexedStringEvenLen.change(33333)
        val borrowed2 = opaqueMutexedStringEvenLen.borrowSelfOrOther(opaqueMutexedStringOddLen)
        assertEquals(borrowed2.getLenAndAdd(0), 3)
    }

    @Test
    fun testStrReturn() {
        val opaque = OpaqueMutexedString.fromUsize(356)
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
