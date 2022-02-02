using System;

using Xunit;

namespace DiplomatFeatures.Tests;

public class ResultTests
{
    [Fact]
    public void ResultOpaqueNew()
    {
        ResultOpaque r = ResultOpaque.New(5);
        r.AssertInteger(5);
    }

    [Fact]
    public void ResultOpaqueNewFailingFoo()
    {
        try
        {
            ResultOpaque.NewFailingFoo();
            Assert.True(false, "expected error didn't occur");
        }
        catch (ErrorEnumException e)
        {
            Assert.Equal(ErrorEnum.Foo, e.Inner);
        }
    }

    [Fact]
    public void ResultOpaqueNewFailingBar()
    {
        try
        {
            ResultOpaque.NewFailingBar();
            Assert.True(false, "expected error didn't occur");
        }
        catch (ErrorEnumException e)
        {
            Assert.Equal(ErrorEnum.Bar, e.Inner);
        }
    }

    [Fact]
    public void ResultOpaqueNewFailingUnit()
    {
        Action act = () => ResultOpaque.NewFailingUnit();
        Assert.Throws<Diplomat.DiplomatOpaqueException>(act);
    }

    [Fact]
    public void ResultOpaqueNewInErr()
    {
        try
        {
            ResultOpaque.NewInErr(198);
            Assert.True(false, "expected error didn't occur");
        }
        catch (ResultOpaqueException e)
        {
            e.Inner.AssertInteger(198);
        }
    }

    [Fact]
    public void ResultOpaqueNewInEnumErr()
    {
        try
        {
            ResultOpaque.NewInEnumErr(989);
            Assert.True(false, "expected error didn't occur");
        }
        catch (ResultOpaqueException e)
        {
            e.Inner.AssertInteger(989);
        }
    }
}
