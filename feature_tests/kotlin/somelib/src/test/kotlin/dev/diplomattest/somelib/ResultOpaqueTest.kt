package dev.gigapixel.somelib

import dev.diplomattest.somelib.*
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class ResultOpaqueTest {

    @Test
    fun testOpaqueResult() {
        val result = ResultOpaque.new_(10)
        val success = result.getOrThrow()
        success.assertInteger(10)

        val resultOpaque2 = ResultOpaque.newFailingBar()
        assert(resultOpaque2.isFailure)

        val result2 = resultOpaque2.exceptionOrNull()?.message
        val shouldRes: Result<ResultOpaque> = ErrorEnum.Bar.err()

        assertEquals(result2, shouldRes.exceptionOrNull()?.message)


        val resultOpaque3 = ResultOpaque.newFailingFoo()
        assert(resultOpaque3.isFailure)
        val result3 = resultOpaque3.exceptionOrNull()?.message
        val shouldRes3: Result<ResultOpaque> = ErrorEnum.Foo.err()
        assertEquals(result3, shouldRes3.exceptionOrNull()?.message)

        val resultOpaque4 = ResultOpaque.newInErr(8)
        assert(resultOpaque4.isFailure)
        val result4 = resultOpaque4.exceptionOrNull()?.message
        val assertion = result4?.startsWith("Received error dev.diplomattest.somelib.ResultOpaque", true)
        assert(assertion == true)
    }

}