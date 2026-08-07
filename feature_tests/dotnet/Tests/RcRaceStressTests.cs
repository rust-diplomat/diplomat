using System;
using System.Runtime.CompilerServices;
using System.Threading;
using System.Threading.Tasks;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Marker collection: groups every test class that touches the shared,
// process-wide Rust drop-count/drop-seq statics behind `RcSource`/
// `RcDependent` (see `feature_tests/src/lifetimes.rs`) so xUnit never runs
// them concurrently with each other — see the comment on
// `RcBorrowDependencyTests` for why that would otherwise corrupt results.
[CollectionDefinition(Name, DisableParallelization = true)]
public class RcSharedNativeStateCollection
{
    public const string Name = "RcSharedNativeState";
}

// Stress-races the .NET-only borrow-dependency RC lifecycle synchronization
// (`RustHandleState<T>` in `tool/templates/dotnet/RustHandle.cs.jinja`)
// under GENUINE concurrency, unlike `RcBorrowDependencyTests` (which is
// single-threaded by design). Two concurrency hazards are exercised many
// times each, because both are inherently timing-dependent and would not
// reliably reproduce in a single run:
//
//  1. Two application threads calling `Dispose()` on the very same wrapper
//     instance at the same time (not just Dispose-then-Dispose, but
//     Dispose-*while*-Dispose) — the "same wrapper owner reference must be
//     one-shot" hazard.
//  2. An application thread calling `source.Dispose()` at the exact instant
//     the GC's own dedicated finalizer thread is concurrently running the
//     *dependent's* finalizer, which releases its retained dependency token
//     on that same shared `RustHandleState<T>` — the actual bug-report
//     hazard: finalizers are concurrent with the application even when the
//     application itself is single-threaded.
//
// Before the lock-guarded redesign, a plain non-atomic `int` refcount could
// lose a decrement under either interleaving (leaking the native
// allocation), or let two threads both observe the count reaching zero and
// both invoke the Rust destructor (double-destroy). Every iteration here
// asserts exactly-once destruction of both the source and the dependent —
// no leak, no double-drop — and that the dependent is always destroyed
// strictly before the source, even under the race.
[Collection(RcSharedNativeStateCollection.Name)]
public class RcRaceStressTests
{
    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static void ForceGcUntil(Func<bool> condition)
    {
        for (int i = 0; i < 100 && !condition(); i++)
        {
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();
        }
    }

    // ── Hazard 1: many application threads racing Dispose() on the SAME ────
    // ── wrapper (both the source and, separately, the dependent) ───────────
    [Fact]
    public void ConcurrentDisposeOnSameWrapper_DestroysExactlyOnce_NoMatterHowManyRacingThreads()
    {
        const int iterations = 200;
        const int threadsPerWrapper = 8;

        for (int iter = 0; iter < iterations; iter++)
        {
            RcSource.ResetDropStats();
            RcDependent.ResetDropStats();

            RcSource source = RcSource.Create((ulong)iter);
            RcDependent dependent = source.MakeDependent();

            var start = new ManualResetEventSlim(false);
            var tasks = new Task[threadsPerWrapper * 2];
            for (int t = 0; t < threadsPerWrapper; t++)
            {
                tasks[t] = Task.Run(() =>
                {
                    start.Wait();
                    source.Dispose();
                });
                tasks[threadsPerWrapper + t] = Task.Run(() =>
                {
                    start.Wait();
                    dependent.Dispose();
                });
            }

            start.Set();
            Task.WaitAll(tasks);
            start.Dispose();

            Assert.Equal(
                1ul,
                RcDependent.DropCount()
            );
            Assert.Equal(
                1ul,
                RcSource.DropCount()
            );

            ulong dependentSeq = RcDependent.DropSeq();
            ulong sourceSeq = RcSource.DropSeq();
            Assert.True(dependentSeq != 0 && sourceSeq != 0);
            Assert.True(
                dependentSeq < sourceSeq,
                $"iteration {iter}: expected dependent (seq {dependentSeq}) destroyed before " +
                $"source (seq {sourceSeq}) even with {threadsPerWrapper} threads racing " +
                "Dispose() on each wrapper concurrently"
            );
        }
    }

    // ── Hazard 2: application-thread source.Dispose() racing the ──────────
    // ── dependent's OWN finalizer on the GC's dedicated finalizer thread ───
    [Fact]
    public void SourceDispose_RacesDependentFinalizer_DestroysExactlyOnce_NoLeak()
    {
        const int iterations = 60;

        for (int iter = 0; iter < iterations; iter++)
        {
            RcSource.ResetDropStats();
            RcDependent.ResetDropStats();

            RcSource source = RcSource.Create((ulong)iter);
            WeakReference dependentRef = CreateUnreferencedDependent(source);

            var ready = new ManualResetEventSlim(false);
            Task gcPressure = Task.Run(() =>
            {
                ready.Wait();
                for (int i = 0; i < 20 && dependentRef.IsAlive; i++)
                {
                    GC.Collect();
                    GC.WaitForPendingFinalizers();
                }
            });

            // Racing on purpose: source.Dispose() runs on THIS thread while
            // the task above concurrently drives the dependent through the
            // GC's finalizer thread — both paths release a reference into
            // the exact same shared RustHandleState<T> at once.
            ready.Set();
            source.Dispose();
            gcPressure.Wait();
            ready.Dispose();

            // The finalizer thread's timing is inherently nondeterministic,
            // so give it a bounded number of extra chances to finish —
            // this loop is about tolerating scheduling jitter, not about
            // the correctness assertion itself (which happens after it).
            ForceGcUntil(() => !dependentRef.IsAlive && RcDependent.DropCount() == 1ul);

            Assert.False(
                dependentRef.IsAlive,
                $"iteration {iter}: the dependent must eventually be collected/finalized"
            );
            Assert.Equal(1ul, RcDependent.DropCount());
            Assert.Equal(1ul, RcSource.DropCount());

            ulong dependentSeq = RcDependent.DropSeq();
            ulong sourceSeq = RcSource.DropSeq();
            Assert.True(dependentSeq != 0 && sourceSeq != 0);
            Assert.True(
                dependentSeq < sourceSeq,
                $"iteration {iter}: expected dependent (seq {dependentSeq}) destroyed before " +
                $"source (seq {sourceSeq}) even when the source's own Dispose() races the " +
                "dependent's finalizer on a different (GC finalizer) thread"
            );
        }
    }

    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static WeakReference CreateUnreferencedDependent(RcSource source)
    {
        RcDependent dependent = source.MakeDependent();
        return new WeakReference(dependent);
    }
}
