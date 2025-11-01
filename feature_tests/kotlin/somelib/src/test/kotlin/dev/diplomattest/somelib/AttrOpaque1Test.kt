package dev.diplomattest.somelib

import io.mockk.* // For mockk, spyk, every, verify
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class AttrOpaque1Test() {

    @Test
    fun testMethod() {
        var mockAttrOpaque1Interface: AttrOpaque1Interface = mockk()
        var attrOpaque1 = AttrOpaque1.new_()

        var expectedUByte = 77.toUByte()
        every {mockAttrOpaque1Interface.method() } returns expectedUByte
        
        var resultWithMock = mockAttrOpaque1Interface.method()
        var resultWithoutMock = attrOpaque1.method()

        assertEquals(resultWithoutMock, resultWithMock)
    }

    @Test
    fun testAbirenamed() {
        var mockAttrOpaque1Interface: AttrOpaque1Interface = mockk()
        var attrOpaque1 = AttrOpaque1.new_()

        var expectedUByte = 123.toUByte()
        every {mockAttrOpaque1Interface.abirenamed() } returns expectedUByte
        
        var resultWithMock = mockAttrOpaque1Interface.abirenamed()
        var resultWithoutMock = attrOpaque1.abirenamed()

        assertEquals(resultWithoutMock, resultWithMock)
    }
}