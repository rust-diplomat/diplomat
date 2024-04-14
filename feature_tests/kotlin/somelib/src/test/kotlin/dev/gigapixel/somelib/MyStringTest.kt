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
        // this should be ancient egyptian for "his brother elder"
        // transcribed from the wikipedia article on "a tale of two brothers
        // used to verify correct function of codepoints beyond U+FFFF
        val ancientEgyptian = "\uD80C\uDD6E\uD80C\uDDCC  \uD80C\uDDBC\uD80C\uDC00  \uD80C\uDE7B\uD80D\uDC30\uD80C\uDFDB"
        myString.setStr(ancientEgyptian)
        assertEquals(ancientEgyptian, myString.getStr())
    }
}