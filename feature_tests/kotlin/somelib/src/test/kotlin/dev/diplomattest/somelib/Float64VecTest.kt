package dev.diplomattest.somelib

import net.jqwik.api.ForAll
import net.jqwik.api.Property
import net.jqwik.api.constraints.NotEmpty
import net.jqwik.api.constraints.Size
import net.jqwik.api.constraints.UniqueElements
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class Float64VecTest {
    @Test
    fun testFloat64Vec() {
        val doubleList = listOf(1.0, 2.0, 3.0, 4.0)
        val doubleArray = doubleList.toDoubleArray()
        val float64Array = Float64Vec.newFromOwned(doubleArray)
        val float64ArrayStr = float64Array.toString()
        assertEquals(float64ArrayStr, doubleList.toString())
    }

}

class Float64PropTest {
    @Property(tries = 1000)
    fun `should generate Lists of Doubles`(
        @ForAll @NotEmpty @Size(value = 1000) numbers: @UniqueElements List<Double>
    ) {
        val doubleArray = numbers.toDoubleArray()
        val float64Array = Float64Vec.newFromOwned(doubleArray)
        val float64List = float64Array.borrow().toList()
        assertEquals(float64List, numbers)
    }

}
