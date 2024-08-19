package dev.diplomattest.somelib

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class CallbackTest {
    @Test
    fun testMultiArgCallback() {
        val cb: (Int ) -> Int = { i -> i + 2 }
        val calledBack = CallbackWrapper.testMultiArgCallback(cb, 10)
        assertEquals(22, calledBack)
    }

    @Test
    fun testNoArgCallback() {
        val cb: () -> Unit = { println("Hello this is a callback"); }
        val calledBack = CallbackWrapper.testNoArgs(cb)
        assertEquals(-5, calledBack)
    }

    @Test
    fun testCbWithStructCallback() {
        val cb: (CallbackTestingStruct) -> Int = { s -> s.x + s.y}
        val calledBack = CallbackWrapper.testCbWithStruct(cb)
        assertEquals(6, calledBack)
    }

    @Test
    fun testMultiCallbacks() {
        val cb1: () -> Int = { 10 }
        val cb2: (Int) -> Int = { x -> x * 2 }
        val calledBack = CallbackWrapper.testMultipleCbArgs(cb1, cb2)
        assertEquals(20, calledBack)
    }
}