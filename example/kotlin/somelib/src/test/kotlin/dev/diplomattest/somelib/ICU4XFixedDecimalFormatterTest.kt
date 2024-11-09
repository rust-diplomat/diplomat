package dev.diplomattest.somelib

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ICU4XFixedDecimalFormatterTest {
    @Test
    fun testFormatter() {
        val locale = Locale.new_("en")
        val provider = DataProvider.newStatic()
        val options = FixedDecimalFormatterOptions.default_()
        val formatter = FixedDecimalFormatter.tryNew(locale, provider, options).getOrThrow()
        val decimal: FixedDecimal = FixedDecimal.new_(123)
        val formatted = formatter.formatWrite(decimal)
        assertEquals(formatted, "123")
    }
}