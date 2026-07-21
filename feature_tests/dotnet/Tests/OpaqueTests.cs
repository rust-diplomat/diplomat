using System;
using System.Text;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Opaque handle lifecycle (IDisposable), string params (`&DiplomatStr`),
// the `string`-returning writer path (`get_debug_str`), nullable-pointer
// Option returns, and passing a value struct into an opaque method.
public class OpaqueTests
{
    [Fact]
    public void New_DebugStrIsEmptyQuoted()
    {
        using Opaque o = Opaque.New();
        // Rust writes `{:?}` of an empty String → the two-character `""`.
        Assert.Equal("\"\"", o.GetDebugStr());
    }

    [Fact]
    public void FromStr_DebugStrIsQuoted()
    {
        using Opaque o = Opaque.FromStr("hello");
        Assert.Equal("\"hello\"", o.GetDebugStr());
    }

    [Fact]
    public void ReturnsUsize_Is412()
    {
        Assert.Equal((nuint)412, Opaque.ReturnsUsize());
    }

    [Fact]
    public void TryFromUtf8_ValidInputReturnsNonNull()
    {
        // `&DiplomatStr` (unvalidated UTF-8) lowers to a zero-copy `byte[]`.
        using Opaque? o = Opaque.TryFromUtf8(Encoding.UTF8.GetBytes("hi"));
        Assert.NotNull(o);
    }

    [Fact]
    public void AssertStruct_AcceptsCanonicalStruct()
    {
        using Opaque o = Opaque.New();
        // Rust-side `assert_value()` panics if any field differs; reaching
        // the next line means the struct round-tripped intact.
        o.AssertStruct(MyStruct.New());
    }

    [Fact]
    public void Dispose_IsIdempotent()
    {
        Opaque o = Opaque.New();
        o.Dispose();
        o.Dispose(); // second Dispose is a no-op, must not double-free
    }

    [Fact]
    public void DisposedOpaqueArgument_ThrowsObjectDisposedException()
    {
        OptionOpaque arg = OptionOpaque.New(5)!;
        arg.Dispose();
        // Disposed arg must throw, not feed a freed pointer to native (UAF).
        Assert.Throws<ObjectDisposedException>(() => OptionOpaque.OptionOpaqueArgument(arg));
    }
}
