package dev.diplomattest.somelib

import org.junit.jupiter.api.Assertions
import org.openjdk.jmh.annotations.Benchmark;
import org.openjdk.jmh.annotations.Scope;
import org.openjdk.jmh.annotations.State;
import org.openjdk.jmh.infra.Blackhole;

@State(Scope.Benchmark)
internal open class ICU4XFixedDecimalFormatterBench {
    private val locale = Locale.new_("en")
    private val provider = DataProvider.newStatic()
    private val options = FixedDecimalFormatterOptions.default_()
    private val formatter = FixedDecimalFormatter.tryNew(locale, provider, options).getOrThrow()
    private val decimal = FixedDecimal.new_(123)

    @Benchmark
    fun benchLocale(bh: Blackhole) {
        bh.consume(Locale.new_("en"))
    }

    @Benchmark
    fun benchProvider(bh: Blackhole) {
        bh.consume(DataProvider.newStatic())
    }

    @Benchmark
    fun benchOptions(bh: Blackhole) {
        bh.consume(FixedDecimalFormatterOptions.default_())
    }

    @Benchmark
    fun benchDecimal(bh: Blackhole) {
        bh.consume(FixedDecimal.new_(123))
    }

    @Benchmark
    fun benchFormatter(bh: Blackhole) {
        bh.consume(FixedDecimalFormatter.tryNew(locale, provider, options).getOrThrow())
    }

    @Benchmark
    fun benchFormat(bh: Blackhole) {
        bh.consume(formatter.formatWrite(decimal))
    }

}