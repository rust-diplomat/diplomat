using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Borrowed opaque returns. `OpaqueThinVec` owns a `Vec<Internal>`; `First()`
// hands back a *borrowed* `OpaqueThin` (`&T`) wrapped in a non-owning
// RustHandle. These tests pin the three things that make that safe: the borrow
// is readable, disposing it never frees Rust's memory (no double-free), and the
// `_edges` root keeps the owner alive while a borrow is still outstanding.
public class BorrowedReturnTests
{
    [Fact]
    public void First_ReturnsReadableBorrowedView()
    {
        using OpaqueThinVec vec = OpaqueThinVec.CreateSingle(7, 1.5f, "hello");
        Assert.Equal((nuint)1, vec.Len());

        using OpaqueThin first = vec.First()!;
        Assert.NotNull(first);
        Assert.Equal(7, first.A());
        Assert.Equal(1.5f, first.B());
        Assert.Equal("hello", first.C());
    }

    [Fact]
    public void Get_InRangeBorrows_OutOfRangeReturnsNull()
    {
        using OpaqueThinVec vec = OpaqueThinVec.CreateSingle(7, 1.5f, "hi");

        // The indexer `get` is a borrowed return just like `First()`: an
        // in-range index hands back a non-owning view into the Vec slot.
        using OpaqueThin at0 = vec.Get(0)!;
        Assert.Equal(7, at0.A());
        Assert.Equal("hi", at0.C());

        // Out-of-range is Rust's `Option::None` path — a null return, not a
        // throw and not a borrow of freed/garbage memory.
        Assert.Null(vec.Get(1));
    }

    [Fact]
    public void First_AliasesOwnerStorage_StringField()
    {
        using OpaqueThinVec vec = OpaqueThinVec.CreateSingle(7, 1.5f, "before");

        // The borrow is taken BEFORE the mutation and never refreshed.
        using OpaqueThin borrow = vec.First()!;
        Assert.Equal("before", borrow.C());

        // Replacing the heap-backed `String` on the owner (which drops the old
        // buffer) is observed through the same outstanding borrow, which
        // re-reads the field. If `First()` had handed back a copy, the borrow
        // would still read "before"; seeing "after" proves it is an interior
        // reference into the same Vec slot the owner just wrote.
        vec.SetFirstC("after");
        Assert.Equal("after", borrow.C());
    }

    [Fact]
    public void DisposingBorrowedView_DoesNotFreeOwnersMemory()
    {
        using OpaqueThinVec vec = OpaqueThinVec.CreateSingle(7, 1.5f, "hi");

        // A borrowed handle owns nothing, so Dispose must be a no-op on Rust's
        // pointer — even called twice, it must not double-free.
        OpaqueThin first = vec.First()!;
        first.Dispose();
        first.Dispose();

        // The owner is untouched: a fresh borrow still reads correctly.
        using OpaqueThin again = vec.First()!;
        Assert.Equal(7, again.A());
    }

    // Tier1's precise liveness can drop the `OpaqueThinVec` local at its last
    // use (the `First()` call). If `_edges` didn't root it, GC -> finalizer ->
    // Destroy would free the Vec out from under the returned borrow.
    // AggressiveOptimization forces that liveness so the race can surface.
    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.AggressiveOptimization)]
    private static OpaqueThin BorrowAndDropOwner()
    {
        return OpaqueThinVec.CreateSingle(42, 2.5f, "rooted").First()!;
    }

    [Fact]
    public void BorrowedView_KeepsOwnerAliveAcrossGc()
    {
        OpaqueThin borrow = BorrowAndDropOwner();

        // Pressure the GC: only `_edges` keeps the now-unreferenced owner alive.
        for (int i = 0; i < 10; i++)
        {
            _ = new byte[256 * 1024];
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        // If the owner had been finalized, these read freed memory (UAF).
        Assert.Equal(42, borrow.A());
        Assert.Equal("rooted", borrow.C());
        GC.KeepAlive(borrow);
    }
}
