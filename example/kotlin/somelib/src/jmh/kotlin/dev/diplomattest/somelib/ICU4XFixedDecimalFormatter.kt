package dev.diplomattest.somelib

import org.junit.jupiter.api.Assertions
import org.openjdk.jmh.annotations.Benchmark;
import org.openjdk.jmh.annotations.Scope;
import org.openjdk.jmh.annotations.State;
import org.openjdk.jmh.infra.Blackhole;

@State(Scope.Benchmark)
internal open class ICU4XFixedDecimalFormatterBench {
    private val locale = ICU4XLocale.new_("en")
    private val provider = ICU4XDataProvider.newStatic()
    private val options = ICU4XFixedDecimalFormatterOptions.default_()
    private val formatter = ICU4XFixedDecimalFormatter.tryNew(locale, provider, options).wrapErrAndThrow()
    private val decimal = ICU4XFixedDecimal.new_(123)

    @Benchmark
    fun benchLocale(bh: Blackhole) {
        bh.consume(ICU4XLocale.new_("en"))
    }

    @Benchmark
    fun benchProvider(bh: Blackhole) {
        bh.consume(ICU4XDataProvider.newStatic())
    }

    @Benchmark
    fun benchOptions(bh: Blackhole) {
        bh.consume(ICU4XFixedDecimalFormatterOptions.default_())
    }

    @Benchmark
    fun benchDecimal(bh: Blackhole) {
        bh.consume(ICU4XFixedDecimal.new_(123))
    }

    @Benchmark
    fun benchFormatter(bh: Blackhole) {
        bh.consume(ICU4XFixedDecimalFormatter.tryNew(locale, provider, options).wrapErrAndThrow())
    }

    @Benchmark
    fun benchFormat(bh: Blackhole) {
        bh.consume(formatter.formatWrite(decimal))
    }

}