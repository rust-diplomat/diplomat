package dev.gigapixel.somelib

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class OtherOpaqueTest {
   @Test
   fun testLoad() {
       val otherOpaque = OtherOpaque.fromUsize(356)
       assertEquals(otherOpaque.getLenAndAdd(4), 7)
       val borrowed = otherOpaque.borrow()
       borrowed.change(1234)
       assertEquals(otherOpaque.getLenAndAdd(4), 8)
   }
}