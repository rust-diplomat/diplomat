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
        var opaque = new Opaque();
        bh.consume(opaque);
        // opaque.delete();
    }

    public static void main(String[] args) throws RunnerException {
        Options opt = new OptionsBuilder()
                .include(OpaqueBench.class.getSimpleName())
                .forks(1)
                .build();

        new Runner(opt).run();
    }
}
