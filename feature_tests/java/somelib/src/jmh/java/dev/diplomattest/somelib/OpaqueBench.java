package dev.diplomattest.somelib;

import org.openjdk.jmh.annotations.Benchmark;
import org.openjdk.jmh.infra.Blackhole;
import org.openjdk.jmh.runner.Runner;
import org.openjdk.jmh.runner.RunnerException;
import org.openjdk.jmh.runner.options.Options;
import org.openjdk.jmh.runner.options.OptionsBuilder;

public class OpaqueBench {
    @Benchmark
    public static void benchOpaque(Blackhole bh) {
        var opaque = Opaque.fromStr("it's amazing to be here");
        bh.consume(opaque);
        // opaque.destroy();
    }

    public static void main(String[] args) throws RunnerException {
        Options opt = new OptionsBuilder()
                .include(OpaqueBench.class.getSimpleName())
                .warmupIterations(2)
                .measurementIterations(2)
                .forks(1)
                .build();

        new Runner(opt).run();
    }
}
