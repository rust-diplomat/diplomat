using System;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class MyStringTests
{
    [Fact]
    public void New_GetStr_RoundTripsUtf8()
    {
        using MyString value = MyString.New("hello 餐");

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
        using MyString value = MyString.New("old");

        value.SetStr("new 餐");

        Assert.Equal("new 餐", value.GetStr());
    }

    [Fact]
    public void StringInputs_RejectNull()
    {
        Assert.Throws<ArgumentNullException>(() => MyString.New(null!));

        using MyString value = MyString.New("value");
        Assert.Throws<ArgumentNullException>(() => value.SetStr(null!));
    }
}
