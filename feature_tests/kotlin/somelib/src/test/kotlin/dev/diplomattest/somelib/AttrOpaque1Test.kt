package dev.diplomattest.somelib

import io.mockk.* // For mockk, spyk, every, verify
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class AttrOpaque1Test() {

    @Test
    fun testMethod() {
        var mockAttrOpaque1Interface: AttrOpaque1RenamedInterface = mockk()
        var attrOpaque1 = AttrOpaque1Renamed.new_()

        var expectedUByte = 77.toUByte()
        every {mockAttrOpaque1Interface.method() } returns expectedUByte
        
        var resultWithMock = mockAttrOpaque1Interface.method()
        var resultWithoutMock = attrOpaque1.method()

        assertEquals(resultWithoutMock, resultWithMock)
    }

    @Test
    fun testAbirenamed() {
        var mockAttrOpaque1Interface: AttrOpaque1RenamedInterface = mockk()
        var attrOpaque1 = AttrOpaque1Renamed.new_()

        var expectedUByte = 123.toUByte()
        every {mockAttrOpaque1Interface.abirenamed() } returns expectedUByte
        
        var resultWithMock = mockAttrOpaque1Interface.abirenamed()
        var resultWithoutMock = attrOpaque1.abirenamed()

        assertEquals(resultWithoutMock, resultWithMock)
    }

    @Test
    fun testStructWithRenamedFields() {
        val initial = RenamedStructWithRenamedFields(
            RenamedStructWithAttrs(true, 42.toUInt()),
            null,
            RenamedAttrEnum.B,
            RenamedAttrEnum.Renamed,
        )

        val withSome = initial.withOptInner(RenamedStructWithAttrs(false, 99.toUInt()))
        assertEquals(true, withSome.inner.a)
        assertEquals(42.toUInt(), withSome.inner.b)
        assertNotNull(withSome.optInner)
        assertEquals(false, withSome.optInner!!.a)
        assertEquals(99.toUInt(), withSome.optInner!!.b)
        assertEquals(RenamedAttrEnum.B, withSome.attrEnum)
        assertEquals(RenamedAttrEnum.Renamed, withSome.optAttrEnum)

        val cleared = RenamedStructWithRenamedFields(
            withSome.inner,
            withSome.optInner,
            RenamedAttrEnum.A,
            null,
        ).withOptInner(null)
        assertNull(cleared.optInner)
        assertEquals(RenamedAttrEnum.A, cleared.attrEnum)
        assertNull(cleared.optAttrEnum)
    }
}