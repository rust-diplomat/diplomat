package dev.diplomattest.somelib

import com.sun.jna.Native
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Test
import javax.swing.text.html.Option
import kotlin.test.assertEquals
import kotlin.test.assertNotNull

class OptionOpaqueTest {
    @Test
    fun testOption() {
        val libClass: Class<OptionOpaqueLib> = OptionOpaqueLib::class.java
        val lib: OptionOpaqueLib = Native.load("diplomat_feature_tests", libClass)
        val ptr = lib.OptionOpaque_new(0)
        val ptr_2 = lib.OptionOpaque_new_none()
        assert(ptr != null)
        assert(ptr_2 == null)

        val newStruct = OptionOpaque.newStruct()
        assertNotNull(newStruct.a)
        newStruct.a?.assertInteger(101)
        assertNotNull(newStruct.b)
        newStruct.b?.assertChar('餐'.code)
        assertEquals(newStruct.c, 904.toUInt())
        assertNotNull(newStruct.d)
        newStruct.d?.assertInteger(926535)

        val noneStruct = OptionOpaque.newStructNones()
        assertNull(noneStruct.a)
        assertNull(noneStruct.b)
        assertEquals(noneStruct.c, 908.toUInt())

    }

    @Test
    fun testPrimitiveOptionReturns() {
        val someOption = OptionOpaque.new_(12) ?: throw RuntimeException("Failed to get option")
        assertEquals(someOption.optionI32(), 10)
        assertEquals(someOption.optionU32(), 10.toUInt())

    }

    @Test
    fun testAcceptsOptionStruct() {
        var maybeStruct = OptionOpaque.acceptsOptionInputStruct(null, 123.toUByte())
        assertEquals(maybeStruct, null)
        val inner = OptionInputStruct.newFromParts(7.toUByte(), null, OptionEnum.Bar)
        maybeStruct = OptionOpaque.acceptsOptionInputStruct(inner, 123.toUByte())
        assertEquals(maybeStruct!!.a, 7.toUByte())
        assertEquals(maybeStruct!!.b, null)
        assertEquals(maybeStruct!!.c, OptionEnum.Bar)

        val returned = OptionOpaque.returnsOptionInputStruct();
        assertEquals(returned.a, 6.toUByte());
        assertEquals(returned.b, null);
        assertEquals(returned.c, OptionEnum.Bar);
    }
}
