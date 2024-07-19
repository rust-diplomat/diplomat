package dev.diplomattest.somelib

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class MyStringTest {

    fun higherOrder( fn: (Int) -> Int, i: Int): Int {
        return fn(i)
    }
    val javaStr = "下面是一句中文"
    val ukrainian = "І це українською мовою"
    // this should be ancient egyptian for "his brother elder"
    // transcribed from the wikipedia article on "a tale of two brothers
    // used to verify correct function of codepoints beyond U+FFFF
    val ancientEgyptian = "\uD80C\uDD6E\uD80C\uDDCC  \uD80C\uDDBC\uD80C\uDC00  \uD80C\uDE7B\uD80D\uDC30\uD80C\uDFDB"
    @Test
    fun testMyString() {
        val i = higherOrder({input -> 2 * input}, 10)
        val myString = MyString.new_(javaStr)
        assertEquals(javaStr, myString.getStr())
        myString.setStr(ukrainian)
        assertEquals(ukrainian, myString.getStr())
        myString.setStr(ancientEgyptian)
        assertEquals(ancientEgyptian, myString.getStr())
    }

    @Test
    fun testMyStringMulti() {
        val javaStrs = arrayOf(ancientEgyptian, ukrainian, javaStr)
        val myString = MyString.newFromFirst(javaStrs)

        assertEquals(ancientEgyptian, myString.getStr())
    }
}