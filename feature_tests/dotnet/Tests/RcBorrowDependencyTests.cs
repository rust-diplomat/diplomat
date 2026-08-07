using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Exercises the .NET-only borrow-dependency reference-counting mechanism
// (see `tool/templates/dotnet/RustHandle.cs.jinja`) directly: a generated
// wrapper that borrows from another opaque retains that source's native
// resource state on construction, and only releases it from its own
// `Cleanup()` — after running its own Rust destructor first, if it has one.
// This defers the source's physical native destruction correctly regardless
// of the order in which the managed wrappers are disposed/finalized.
//
// These tests are single-threaded by design (each test method's own
// Dispose()/finalizer calls happen sequentially on one thread) — for
// GENUINE concurrent-Dispose/finalizer-race coverage of the same underlying
// mechanism, see `RcRaceStressTests`.
//
// Both this class and `RcRaceStressTests` read/reset the SAME global Rust
// drop-count/drop-seq statics (`RC_SOURCE_DROPS`, `RC_DEPENDENT_DROPS`, ...
// in `feature_tests/src/lifetimes.rs`) via `RcSource`/`RcDependent`'s static
// methods — there is exactly one of each counter per process, not one per
// test. Grouping both classes into the same non-parallelized xUnit
// collection (see `RcSharedNativeStateCollection` below) keeps xUnit from
// running them concurrently against each other, which would otherwise
// corrupt each test's own view of "how many times did X drop" with counts
// from a completely unrelated test's objects.
[Collection(RcSharedNativeStateCollection.Name)]
public class RcBorrowDependencyTests
{
    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static void ForceGcUntil(Func<bool> condition)
    {
        for (int i = 0; i < 50 && !condition(); i++)
        {
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();
        }
    }

    private static void ResetAllDropStats()
    {
        RcSource.ResetDropStats();
        RcDependent.ResetDropStats();
        RcDependent2.ResetDropStats();
        RcFinalizerSource.ResetDropStats();
        RcFinalizerDependent.ResetDropStats();
    }

    // ── Borrowed view / source explicit Dispose (opt-in IDisposable) ───────

    [Fact]
    public void BorrowedView_KeepsSourceNativeAllocationAlive_AfterSourceDispose()
    {
        ResetAllDropStats();

        RcSource source = RcSource.Create(42);
        RcSource view = source.View();

        // Disposing the source makes the source *wrapper* unusable, but the
        // native RcSource allocation must stay alive because `view` still
        // holds a retained dependency on it.
        source.Dispose();
        Assert.Throws<ObjectDisposedException>(() => source.Id());
        Assert.Equal(0ul, RcSource.DropCount());

        // The view is a distinct managed wrapper still backed by the same
        // (still-alive) native allocation.
        Assert.Equal(42ul, view.Id());
        Assert.Equal(0ul, RcSource.DropCount());

        // Only once the last reference (the view) is released does the
        // native allocation actually get destroyed.
        view.Dispose();
        Assert.Equal(1ul, RcSource.DropCount());

        Assert.Throws<ObjectDisposedException>(() => view.Id());
    }

    [Fact]
    public void BorrowedView_DoubleDispose_IsIdempotent_AndDropsExactlyOnce()
    {
        ResetAllDropStats();

        RcSource source = RcSource.Create(7);
        RcSource view = source.View();

        view.Dispose();
        view.Dispose(); // idempotent: no double-release, no throw
        Assert.Equal(0ul, RcSource.DropCount()); // source's own ref still held

        source.Dispose();
        source.Dispose(); // idempotent
        Assert.Equal(1ul, RcSource.DropCount());
    }

    // ── Owned-borrowing: dependent's own destructor runs before source ─────

    [Fact]
    public void OwnedBorrowingDependent_DestroysItselfBeforeSource_EvenWhenSourceDisposedFirst()
    {
        ResetAllDropStats();

        RcSource source = RcSource.Create(1);
        RcDependent dependent = source.MakeDependent();

        // Dispose the source *first*, in "outer to inner" order — the user
        // doesn't need to know about the dependency to get this right.
        source.Dispose();
        Assert.Equal(0ul, RcSource.DropCount()); // deferred: dependent still holds a ref
        Assert.Equal(0ul, RcDependent.DropCount());

        // Disposing the dependent must run its own Rust destructor first,
        // and only then release its retained dependency on the source,
        // which finally drops the source's native allocation.
        dependent.Dispose();
        Assert.Equal(1ul, RcDependent.DropCount());
        Assert.Equal(1ul, RcSource.DropCount());

        ulong dependentSeq = RcDependent.DropSeq();
        ulong sourceSeq = RcSource.DropSeq();
        Assert.True(dependentSeq != 0 && sourceSeq != 0);
        Assert.True(
            dependentSeq < sourceSeq,
            $"expected dependent (seq {dependentSeq}) to be destroyed before source (seq {sourceSeq})"
        );
    }

    // ── Transitive/direct dependency chain (only direct edges recorded) ────

    [Fact]
    public void TransitiveChain_DestroysInnermostDependentFirst_RegardlessOfDisposeOrder()
    {
        ResetAllDropStats();

        RcSource source = RcSource.Create(100);
        RcDependent dependent = source.MakeDependent();
        RcDependent2 dependent2 = dependent.MakeDependent2();

        // Dispose "outer to inner": source, then dependent, then dependent2.
        // Each generator-emitted edge is direct (dependent2 -> dependent,
        // dependent -> source); the correct full-chain ordering falls out
        // of each layer's own recursive Release(), not from any transitive
        // bookkeeping in the generator.
        source.Dispose();
        Assert.Equal(0ul, RcSource.DropCount());

        dependent.Dispose();
        Assert.Equal(0ul, RcDependent.DropCount()); // dependent2 still holds a ref
        Assert.Equal(0ul, RcSource.DropCount());

        dependent2.Dispose();
        Assert.Equal(1ul, RcDependent2.DropCount());
        Assert.Equal(1ul, RcDependent.DropCount());
        Assert.Equal(1ul, RcSource.DropCount());

        ulong dependent2Seq = RcDependent2.DropSeq();
        ulong dependentSeq = RcDependent.DropSeq();
        ulong sourceSeq = RcSource.DropSeq();
        Assert.True(dependent2Seq < dependentSeq, "dependent2 must be destroyed before dependent");
        Assert.True(dependentSeq < sourceSeq, "dependent must be destroyed before source");
    }

    // ── Exactly-once destruction under repeated Dispose() ───────────────────

    [Fact]
    public void DependentAndSource_EachDropExactlyOnce_NoMatterHowManyDisposeCalls()
    {
        ResetAllDropStats();

        RcSource source = RcSource.Create(9);
        RcDependent dependent = source.MakeDependent();

        dependent.Dispose();
        dependent.Dispose();
        dependent.Dispose();
        source.Dispose();
        source.Dispose();

        Assert.Equal(1ul, RcDependent.DropCount());
        Assert.Equal(1ul, RcSource.DropCount());
    }

    // ── Finalizer-only (default, non-opt-in) parent/child ordering ─────────

    [Fact]
    public void FinalizerOnlyProbes_AreNotIDisposable()
    {
        Assert.DoesNotContain(typeof(IDisposable), typeof(RcFinalizerSource).GetInterfaces());
        Assert.DoesNotContain(typeof(IDisposable), typeof(RcFinalizerDependent).GetInterfaces());
    }

    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static (WeakReference sourceRef, WeakReference dependentRef) CreateFinalizerPairAndDropReferences()
    {
        RcFinalizerSource source = RcFinalizerSource.Create(55);
        RcFinalizerDependent dependent = source.MakeDependent();
        return (new WeakReference(source), new WeakReference(dependent));
    }

    [Fact]
    public void FinalizerOnlyPair_DependentDestroyedBeforeSource_ViaFinalizers()
    {
        ResetAllDropStats();

        (WeakReference sourceRef, WeakReference dependentRef) = CreateFinalizerPairAndDropReferences();

        ForceGcUntil(() =>
            !sourceRef.IsAlive
            && !dependentRef.IsAlive
            && RcFinalizerSource.DropCount() == 1ul
            && RcFinalizerDependent.DropCount() == 1ul
        );

        Assert.False(sourceRef.IsAlive);
        Assert.False(dependentRef.IsAlive);
        Assert.Equal(1ul, RcFinalizerSource.DropCount());
        Assert.Equal(1ul, RcFinalizerDependent.DropCount());

        ulong dependentSeq = RcFinalizerDependent.DropSeq();
        ulong sourceSeq = RcFinalizerSource.DropSeq();
        Assert.True(dependentSeq != 0 && sourceSeq != 0);
        Assert.True(
            dependentSeq < sourceSeq,
            $"expected finalizer-only dependent (seq {dependentSeq}) to be destroyed before source (seq {sourceSeq})"
        );
    }
}
