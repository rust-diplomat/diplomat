using System;
using System.Runtime.CompilerServices;
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

        public int ReleaseCount => _releaseCount;

        public void Release() => _releaseCount++;
    }

    // Pointer is never dereferenced on the oversize path; only len + edges matter.
    [MethodImpl(MethodImplOptions.NoInlining)]
    private static unsafe void ConstructOversizeBorrowedSpan(ILifetimeEdge?[] edges)
    {
        nuint oversize = (nuint)int.MaxValue + 1;
        _ = new DiplomatBorrowedSpan<byte>(null, oversize, edges);
    }

    [Fact]
    public void OversizeConstructor_ThrowsAndReleasesBorrowEdgesImmediately()
    {
        var edge = new CountingEdge();
        ILifetimeEdge?[] edges = new ILifetimeEdge?[] { edge };

        IndexOutOfRangeException ex = Assert.Throws<IndexOutOfRangeException>(() =>
            ConstructOversizeBorrowedSpan(edges)
        );
        Assert.Contains("too large", ex.Message);

        // Must not require a GC pass: failed construction owns the cleanup duty
        // for edges it was handed.
        Assert.Equal(1, edge.ReleaseCount);
    }
}
