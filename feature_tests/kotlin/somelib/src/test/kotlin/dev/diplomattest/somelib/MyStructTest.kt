package dev.diplomattest.somelib

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class MyStructTest {
    @Test
    fun testMyStruct() {
        val struct: MyStruct = MyStruct.new_()
        assertEquals(struct.a, 17.toUByte())
        assertEquals(struct.g, MyEnum.B)
    }
}