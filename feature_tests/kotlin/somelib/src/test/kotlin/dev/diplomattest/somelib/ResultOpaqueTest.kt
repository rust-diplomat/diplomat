package dev.diplomattest.somelib

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
        val shouldRes: Result<ResultOpaque> = ErrorEnumError(ErrorEnum.Bar).err()

        assertEquals(result2, shouldRes.exceptionOrNull()?.message)


        val resultOpaque3 = ResultOpaque.newFailingFoo()
        assert(resultOpaque3.isFailure)
        val result3 = resultOpaque3.exceptionOrNull()?.message
        val shouldRes3: Result<ResultOpaque> = ErrorEnumError(ErrorEnum.Foo).err()
        assertEquals(result3, shouldRes3.exceptionOrNull()?.message)

        val resultOpaque4 = ResultOpaque.newInErr(8)
        assert(resultOpaque4.isFailure)
        val result4 = resultOpaque4.exceptionOrNull()?.message
        val assertion = result4?.startsWith("Rust error result for ResultOpaque", true)
        assert(assertion == true)

        val resultOpaque5 = ResultOpaque.newFailingInt(5)
        assert(resultOpaque5.isFailure)
        try {
            resultOpaque5.getOrThrow()
            assert(false == true) // should not reach here
        } catch(ie: IntError) {
            assertEquals(ie.getValue(), 5)
        }
    }

}
