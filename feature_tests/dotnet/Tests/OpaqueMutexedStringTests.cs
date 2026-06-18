using Somelib;
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
}
