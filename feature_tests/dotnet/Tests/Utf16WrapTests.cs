using System;
using System.Runtime.CompilerServices;
using Somelib;
using Somelib.Diplomat;
using Xunit;

namespace Somelib.FeatureTests;

// `&DiplomatStr16` inputs and `&'a DiplomatStr16` returns: a C# `string` is
// already a flat UTF-16 buffer, so both directions are zero-copy — no
// transcoding, unlike the UTF-8 (`&DiplomatStr`/`&str`) path.
public class Utf16WrapTests
{
    [Fact]
    public void FromUtf16_GetDebugStr_ShowsUtf16CodeUnits()
    {
        using Utf16Wrap value = Utf16Wrap.FromUtf16("AB");

        Assert.Equal("[65, 66]", value.GetDebugStr());
    }

    // `fixed` on a C# string — even `""` — pins a valid pointer to its null
    // terminator; `Len = 0` means Rust never reads through it. This is the
    // case the old UTF-8-only path couldn't offer: `fixed` over an empty
    // `byte[]` binds a null pointer instead.
    [Fact]
    public void FromUtf16_EmptyString_Works()
    {
        using Utf16Wrap value = Utf16Wrap.FromUtf16("");

        Assert.Equal("[]", value.GetDebugStr());
    }

    [Fact]
    public void FromUtf16_RejectsNull()
    {
        Assert.Throws<ArgumentNullException>(() => Utf16Wrap.FromUtf16(null!));
    }

    // `borrow_cont` returns `&'a DiplomatStr16` — a zero-copy
    // `DiplomatBorrowedSpan<char>` view into the owner's Rust-side storage.
    // `WithSpan` scopes access to the callback, mirroring `RustVec`.
    [Fact]
    public void BorrowCont_WithSpan_MatchesSourceString()
    {
        using Utf16Wrap value = Utf16Wrap.FromUtf16("hello 𐐷");

        DiplomatBorrowedSpan<char> view = value.BorrowCont();

        string decoded = "";
        view.WithSpan(span => decoded = new string(span));
        Assert.Equal("hello 𐐷", decoded);
    }

    // Same liveness trap as MyStringTests.Borrow_KeepsOwnerAliveAcrossGc: if
    // the borrowed view's `_edges` didn't root the owner, GC -> finalizer ->
    // Destroy would free it out from under the view.
    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.AggressiveOptimization)]
    private static DiplomatBorrowedSpan<char> BorrowContAndDropOwner()
    {
        return Utf16Wrap.FromUtf16("rooted 𐐷").BorrowCont();
    }

    [Fact]
    public void BorrowCont_KeepsOwnerAliveAcrossGc()
    {
        DiplomatBorrowedSpan<char> view = BorrowContAndDropOwner();

        for (int i = 0; i < 10; i++)
        {
            _ = new byte[256 * 1024];
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        string decoded = "";
        view.WithSpan(span => decoded = new string(span));
        Assert.Equal("rooted 𐐷", decoded);
        GC.KeepAlive(view);
    }
}
