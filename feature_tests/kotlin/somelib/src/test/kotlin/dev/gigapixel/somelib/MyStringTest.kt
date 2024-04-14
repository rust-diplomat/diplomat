package dev.gigapixel.somelib

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class MyStringTest {
    @Test
    fun testMyString() {
        val javaStr = "下面是一句中文"
        val myString = MyString.new_(javaStr)
        assertEquals(javaStr, myString.getStr())
        val ukrainian = "І це українською мовою"
        myString.setStr(ukrainian)
        assertEquals(ukrainian, myString.getStr())
    }
}