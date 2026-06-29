using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// .NET's GC can finalize the Vec owner while a borrowed handle is still reachable
// if `_edges` doesn't root it — these tests verify that can't happen.
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

    // Same liveness trap, but the borrow comes back through a
    // `Result<&OpaqueThin, ()>` — the keep-alive edges must ride on the
    // *success* (`result.Ok`) wrapper for a fallible return too.
    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.AggressiveOptimization)]
    private static OpaqueThin TryFirstAndDropOwner()
    {
        return OpaqueThinVec.CreateSingle(42, 2.5f, "rooted").TryFirst(false);
    }

    [Fact]
    public void FallibleBorrowedReturn_Ok_KeepsOwnerAliveAcrossGc()
    {
        OpaqueThin borrow = TryFirstAndDropOwner();

        for (int i = 0; i < 10; i++)
        {
            _ = new byte[256 * 1024];
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        // The owner is referenced only through the Ok wrapper's edges; if those
        // weren't wired, the Vec would be finalized and this would be a UAF.
        Assert.Equal(42, borrow.A());
        Assert.Equal("rooted", borrow.C());
        GC.KeepAlive(borrow);
    }

    [Fact]
    public void FallibleBorrowedReturn_Err_Throws()
    {
        using OpaqueThinVec vec = OpaqueThinVec.CreateSingle(7, 1.5f, "hi");
        // The `Err(())` arm throws — and must not hand back a wrapper at all.
        Assert.Throws<InvalidOperationException>(() => vec.TryFirst(true));
    }

    [Fact]
    public void FallibleOptionalBorrowedReturn_Composes()
    {
        using OpaqueThinVec vec = OpaqueThinVec.CreateSingle(7, 1.5f, "hi");

        // Result + Option + borrowing view: Ok(Some(_)) reads through the borrow.
        using OpaqueThin at0 = vec.TryGet(0, false)!;
        Assert.Equal(7, at0.A());

        // Ok(None): out-of-range index is a null return, not a throw.
        Assert.Null(vec.TryGet(5, false));

        // Err(()): the failure arm still throws.
        Assert.Throws<InvalidOperationException>(() => vec.TryGet(0, true));
    }

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.AggressiveOptimization)]
    private static OpaqueThin TryGetAndDropOwner()
    {
        return OpaqueThinVec.CreateSingle(42, 2.5f, "rooted").TryGet(0, false)!;
    }

    [Fact]
    public void FallibleOptionalBorrowedReturn_Ok_KeepsOwnerAliveAcrossGc()
    {
        OpaqueThin borrow = TryGetAndDropOwner();

        for (int i = 0; i < 10; i++)
        {
            _ = new byte[256 * 1024];
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        Assert.Equal(42, borrow.A());
        Assert.Equal("rooted", borrow.C());
        GC.KeepAlive(borrow);
    }

    // `Result<Box<OpaqueThinIter<'a>>, ()>` — the Ok value is an *owned* Box
    // (IronRDP's shape) that still borrows the Vec. Its keep-alive edges must
    // root the now-unreferenced owner, so the owned-but-borrowing case works.
    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.AggressiveOptimization)]
    private static OpaqueThinIter TryIterAndDropOwner()
    {
        return OpaqueThinVec.CreateSingle(42, 2.5f, "rooted").TryIter(false);
    }

    [Fact]
    public void FallibleOwnedBorrowingBoxReturn_Ok_KeepsOwnerAliveAcrossGc()
    {
        OpaqueThinIter iter = TryIterAndDropOwner();

        for (int i = 0; i < 10; i++)
        {
            _ = new byte[256 * 1024];
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        // Reading through the iterator touches the Vec; the only thing keeping
        // that Vec alive is the owned iterator's edges. If they weren't wired,
        // the Vec would be finalized and this a UAF.
        using OpaqueThin first = iter.Next()!;
        Assert.Equal(42, first.A());
        Assert.Null(iter.Next()); // single-element vec: exhausted after first Next()
        GC.KeepAlive(iter);
    }

    [Fact]
    public void FallibleOwnedBorrowingBoxReturn_Err_Throws()
    {
        using OpaqueThinVec vec = OpaqueThinVec.CreateSingle(7, 1.5f, "hi");
        Assert.Throws<InvalidOperationException>(() => vec.TryIter(true));
    }

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.AggressiveOptimization)]
    private static OpaqueThinIter OptionalIterAndDropOwner()
    {
        return OpaqueThinVec.CreateSingle(42, 2.5f, "rooted").OptionalIter(true)!;
    }

    [Fact]
    public void OptionalOwnedBorrowingBoxReturn_Some_KeepsOwnerAliveAcrossGc()
    {
        OpaqueThinIter iter = OptionalIterAndDropOwner();

        for (int i = 0; i < 10; i++)
        {
            _ = new byte[256 * 1024];
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        using OpaqueThin first = iter.Next()!;
        Assert.Equal(42, first.A());
        Assert.Equal("rooted", first.C());
        Assert.Null(iter.Next());
        GC.KeepAlive(iter);
    }

    [Fact]
    public void OptionalOwnedBorrowingBoxReturn_None_ReturnsNull()
    {
        using OpaqueThinVec vec = OpaqueThinVec.CreateSingle(7, 1.5f, "hi");
        Assert.Null(vec.OptionalIter(false));
    }
}
