package dev.gigapixel.somelib

import dev.diplomattest.somelib.*
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class ResultOpaqueTest {

    @Test
    fun testOpaqueResult() {
        val result = ResultOpaque.new_(10)
        val success = result.wrapErrAndThrow()
        success.assertInteger(10)

        val result2 = ResultOpaque.newFailingBar()
        when (result2) {
            is Ok -> assert(false)
            is Err -> assertEquals(result2.inner, ErrorEnum.Bar)
        }


        val result3 = ResultOpaque.newFailingFoo()

        when (result3) {
            is Ok -> assert(false)
            is Err -> assertEquals(result3.inner, ErrorEnum.Foo)
        }

        val result4 = ResultOpaque.newInErr(8)

        when (result4) {
            is Ok -> assert(false)
            is Err -> result4.inner.assertInteger(8)
        }
    }
}