package dev.diplomattest.somelib

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class Float64VecTest {
    @Test
    fun testFloat64Vec() {
        val doubleList = listOf(1.0, 2.0, 3.0, 4.0)
        val float64Array = Float64Vec.new_(doubleList.toDoubleArray())
        assertEquals(float64Array.toString_(), doubleList.toString())
    }
}