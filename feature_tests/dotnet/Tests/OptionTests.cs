using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class OptionTests
{
    [Fact]
    public void NewSome_ReturnsOpaque()
    {
        using OptionOpaque value = Assert.IsType<OptionOpaque>(OptionOpaque.New(23));

        value.AssertInteger(23);
    }

    [Fact]
    public void NewNone_ReturnsNull()
    {
        Assert.Null(OptionOpaque.NewNone());
    }

    [Fact]
    public void PrimitiveOptions_ReturnExpectedValues()
    {
        using OptionOpaque value = Assert.IsType<OptionOpaque>(OptionOpaque.New(1));

        Assert.Equal((nint?)10, value.OptionIsize());
        Assert.Equal((nuint?)10, value.OptionUsize());
        Assert.Equal(10, value.OptionI32());
        Assert.Equal((uint?)10, value.OptionU32());
    }

    [Fact]
    public void OptionOpaqueArgument_MapsNullAndSome()
    {
        using OptionOpaque value = Assert.IsType<OptionOpaque>(OptionOpaque.New(5));

        Assert.True(OptionOpaque.OptionOpaqueArgument(value));
        Assert.False(OptionOpaque.OptionOpaqueArgument(null));
    }

    [Fact]
    public void OptionString_Write_RoundTripsUtf8()
    {
        using OptionString value = Assert.IsType<OptionString>(OptionString.New("hello 餐"));

        Assert.Equal("hello 餐", value.Write());
    }
}
