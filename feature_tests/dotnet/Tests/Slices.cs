using System;

using Xunit;

namespace DiplomatFeatures.Tests;

public class SliceTests
{
    [Fact]
    public void NewMyString()
    {
        MyString s = MyString.New("Hello!");
        Assert.Equal("Hello!", s.GetStr());
        s.SetStr("World!");
        Assert.Equal("World!", s.GetStr());
    }

    [Fact]
    public void MakeUppercase()
    {
        Assert.Equal("HELLO!", MyString.MakeUppercase("Hello!"));
    }

    [Fact]
    public void Float64VecNew()
    {
        double[] expected = new double[] { 1, 2, 3, 4, 5 };
        double[] buffer = new double[5];
        Float64Vec v = Float64Vec.New(expected);
        v.FillSlice(buffer);
        Assert.Equal(buffer, expected);
    }

    [Fact]
    public void Float64VecSetValue()
    {
        double[] initial = new double[] { 1, 2, 3, 4, 5 };
        double[] expected = new double[] { 5, 4, 3, 4, 5 };
        double[] buffer = new double[5];
        Float64Vec v = Float64Vec.New(initial);
        v.SetValue(expected);
        v.FillSlice(buffer);
        Assert.Equal(buffer, expected);
    }
}
