package dev.diplomattest.somelib

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class MyIteratorTest {
    @Test
    fun testIterator() {
        val list = listOf(0, 1, 2, 3, 7, 4)
        val array = list.map { it.toByte().toUByte()}.toUByteArray()

        val myIterable = MyIterable.new_(array)
        val mutableList : MutableList<Int> = mutableListOf()
        val myIterator = myIterable.iterator()
        for (it in myIterator) {
            mutableList.add(it.toInt())
        }
        assertEquals(mutableList.toList(), list)

    }

    @Test
    fun testEmptyIterator() {
        val list: List<Int> = listOf()
        val array = list.map { it.toByte().toUByte()}.toUByteArray()

        val myIterable = MyIterable.new_(array)
        val mutableList : MutableList<Int> = mutableListOf()
        val myIterator = myIterable.iterator()
        for (it in myIterator) {
            mutableList.add(it.toInt())
        }
        assertEquals(mutableList.toList(), list)
    }
}