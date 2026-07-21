using System;
using System.Runtime.CompilerServices;
using System.Text;
using Somelib;
using Somelib.Diplomat;
using Xunit;

namespace Somelib.FeatureTests;

public class MyStringTests
{
    // `New`/`SetStr` take `&DiplomatStr` (unvalidated UTF-8), lowered to a
    // zero-copy `byte[]` param. `NewUnsafe` takes a validated `&str`, which
    // stays `string` and transcodes internally via `Diplomat.Utf8.Clone`.
    private static byte[] Utf8(string s) => Encoding.UTF8.GetBytes(s);

    [Fact]
    public void New_GetStr_RoundTripsUtf8()
    {
        using MyString value = MyString.New(Utf8("hello 餐"));

        Assert.Equal("hello 餐", value.GetStr());
    }

    [Fact]
    public void NewUnsafe_GetStr_RoundTripsUtf8()
    {
        using MyString value = MyString.NewUnsafe("unsafe 𐐷");

        Assert.Equal("unsafe 𐐷", value.GetStr());
    }

    [Fact]
    public void SetStr_ReplacesValue()
    {
        using MyString value = MyString.New(Utf8("old"));

        value.SetStr(Utf8("new 餐"));

        Assert.Equal("new 餐", value.GetStr());
    }

    [Fact]
    public void StringInputs_RejectNull()
    {
        Assert.Throws<ArgumentNullException>(() => MyString.New(null!));

        using MyString value = MyString.New(Utf8("value"));
        Assert.Throws<ArgumentNullException>(() => value.SetStr(null!));
    }

    // `borrow` returns `&'a str` — a zero-copy `DiplomatBorrowedSpan<byte>`
    // view into the owner's Rust-side storage, not a copy. `WithSpan` scopes
    // access to the callback, mirroring `RustVec`.
    [Fact]
    public void Borrow_WithSpan_MatchesGetStr()
    {
        using MyString value = MyString.New(Utf8("hello 餐"));

        DiplomatBorrowedSpan<byte> view = value.Borrow();

        string decoded = "";
        view.WithSpan(span => decoded = Encoding.UTF8.GetString(span));
        Assert.Equal("hello 餐", decoded);
    }

    // Tier1's precise liveness can drop the owning `MyString` local at its
    // last use (the `Borrow()` call). If the view's `_edges` didn't root it,
    // GC -> finalizer -> Destroy would free the owner out from under the
    // borrowed view. AggressiveOptimization forces that liveness so the race
    // can surface.
    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.AggressiveOptimization)]
    private static DiplomatBorrowedSpan<byte> BorrowAndDropOwner()
    {
        return MyString.New(Utf8("rooted 餐")).Borrow();
    }

    [Fact]
    public void Borrow_KeepsOwnerAliveAcrossGc()
    {
        DiplomatBorrowedSpan<byte> view = BorrowAndDropOwner();

        for (int i = 0; i < 10; i++)
        {
            _ = new byte[256 * 1024];
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        // If the owner had been finalized, this reads freed memory (UAF).
        string decoded = "";
        view.WithSpan(span => decoded = Encoding.UTF8.GetString(span));
        Assert.Equal("rooted 餐", decoded);
        GC.KeepAlive(view);
    }
}
