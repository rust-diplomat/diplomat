using System;
using System.Runtime.CompilerServices;
using System.Threading;
using Somelib.Diplomat;
using Xunit;

namespace Somelib.FeatureTests;

// Repro for the oversize DiplomatBorrowedSpan constructor path.
//
// Call sites build source edges *before* `new DiplomatBorrowedSpan(...)`. If
// construction throws because `len` does not fit a .NET Span without releasing
// those edges, the source's borrow stays open and its pins leak.
//
// We use a pure-managed counting edge so this is deterministic and does not
// fight process-global native drop counters from other tests.
public class BorrowedSpanOversizeTests
{
    private sealed class CountingEdge : ILifetimeEdge
    {
        private int _releaseCount;

        public int DisposeCount => Volatile.Read(ref _releaseCount);

        public void Release() => Interlocked.Increment(ref _releaseCount);
    }

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

    // Pointer is never dereferenced on the oversize path; only len + edges matter.
    [MethodImpl(MethodImplOptions.NoInlining)]
    private static unsafe void ConstructOversizeBorrowedSpan(ILifetimeEdge?[] edges)
    {
        nuint oversize = (nuint)int.MaxValue + 1;
        _ = new DiplomatBorrowedSpan<byte>(null, oversize, edges);
    }

    [Fact]
    public void OversizeConstructor_ThrowsAndDisposesBorrowEdgesImmediately()
    {
        var edge = new CountingEdge();
        ILifetimeEdge?[] edges = new ILifetimeEdge?[] { edge };

        IndexOutOfRangeException ex = Assert.Throws<IndexOutOfRangeException>(() =>
            ConstructOversizeBorrowedSpan(edges)
        );
        Assert.Contains("too large", ex.Message);

        // Must not require a GC pass: failed construction owns the cleanup duty
        // for edges it was handed.
        Assert.Equal(1, edge.DisposeCount);
    }

    /// <summary>
    /// Even after the half-built object becomes unreachable, the edge must end
    /// up released. A correct ctor does this before throw; a broken one never does.
    /// </summary>
    [Fact]
    public void OversizeConstructor_FailedObjectDoesNotLeakRetainAcrossGc()
    {
        var edge = new CountingEdge();
        ILifetimeEdge?[] edges = new ILifetimeEdge?[] { edge };

        try
        {
            ConstructOversizeBorrowedSpan(edges);
        }
        catch (IndexOutOfRangeException)
        {
            // expected once the length guard exists
        }

        // Drop managed refs that could keep the edge reachable outside the span.
        edges = null!;

        ForceGcUntil(() => edge.DisposeCount >= 1);

        Assert.Equal(1, edge.DisposeCount);
    }
}
