package dev.diplomattest.somelib

import com.sun.jna.Native
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull

class OptionOpaqueTest {
    @Test
    fun testOption() {
        val libClass: Class<OptionOpaqueLib> = OptionOpaqueLib::class.java
        val lib: OptionOpaqueLib = Native.load("somelib", libClass)
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
        assertNull(noneStruct.d)

    }
}