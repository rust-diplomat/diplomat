using System.Text;
using Somelib;
using Somelib.Diplomat;
using Xunit;

namespace Somelib.FeatureTests;

public class OpaqueMutexedStringTests
{
    [Fact]
    public void Change_UpdatesStoredStringLength()
    {
        using OpaqueMutexedString value = OpaqueMutexedString.FromUsize(356);

        Assert.Equal((nuint)7, value.GetLenAndAdd(4));

        value.Change(1234);

        Assert.Equal((nuint)8, value.GetLenAndAdd(4));
    }

    [Fact]
    public void Wrapper_ReturnsUtf16Opaque()
    {
        using OpaqueMutexedString value = OpaqueMutexedString.FromUsize(356);
        using Utf16Wrap wrapper = value.Wrapper();

        string debug = wrapper.GetDebugStr();

        Assert.StartsWith("[65, 32, 99, 111", debug);
        Assert.Contains("55297, 56375", debug);
    }

    [Fact]
    public void ToUnsignedFromUnsigned_RoundTripsValue()
    {
        using OpaqueMutexedString value = OpaqueMutexedString.FromUsize(356);

        Assert.Equal((ushort)42, value.ToUnsignedFromUnsigned(42));
    }

    // `dummy_str` returns `&'a DiplomatStr` — a zero-copy view into Rust-owned
    // memory. `WithSpan` scopes access to the callback (mirroring `RustVec`);
    // decoding to a real `string` is a separate, explicit
    // `Encoding.UTF8.GetString` call inside it.
    [Fact]
    public void DummyStr_WithSpan_DecodesToExpectedUtf8()
    {
        using OpaqueMutexedString value = OpaqueMutexedString.FromUsize(356);

        DiplomatBorrowedSpan<byte> view = value.DummyStr();

        string decoded = "";
        view.WithSpan(span => decoded = Encoding.UTF8.GetString(span));
        Assert.Equal("A const str with non byte char: 餐 which is a DiplomatChar,", decoded);
    }
}
