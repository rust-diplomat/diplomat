package dev.diplomattest.somelib

import org.junit.jupiter.api.Test

class ICU4XFixedDecimalFormatterTest {
    @Test
    fun testFormatter() {
        val locale = ICU4XLocale.new_("en")
        val provider = ICU4XDataProvider.newStatic()
        val options = ICU4XFixedDecimalFormatterOptions.default_()
        val formatter = ICU4XFixedDecimalFormatter.tryNew(locale, provider, options).wrapErrAndThrow()
        val decimal: ICU4XFixedDecimal = ICU4XFixedDecimal.new_(123)
        val formatted = formatter.formatWrite(decimal)
        println(formatted)
    }
}