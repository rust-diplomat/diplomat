using System;

using Xunit;

namespace DiplomatFeatures.Tests;

public class OptionTests
{
    [Fact]
    public void NewWithValue()
    {
        OptionOpaque? o = OptionOpaque.New(1415);
        Assert.NotNull(o);
        if (o != null) // <-- to suppress warning
        {
            o.AssertInteger(1415);
        }
    }

    [Fact]
    public void NewNoneReturnsNone()
    {
        OptionOpaque? o = OptionOpaque.NewNone();
        Assert.Null(o);
    }

    [Fact]
    public void NewStruct()
    {
        OptionStruct o = OptionOpaque.NewStruct();

        Assert.Equal((ulong)904, o.C);
        // TODO: managed custom types in non-opaque struct fields

        // Using user-provided methods
        o.AssertIntegerForA(101);
        o.AssertCharForB(0x9910);
        o.AssertIntegerForD(926535);
    }

    [Fact]
    public void NewStructNones()
    {
        OptionStruct o = OptionOpaque.NewStructNones();

        Assert.Equal((ulong)908, o.C);
        // TODO: managed custom types in non-opaque struct fields

        // Using user-provided methods
        Assert.True(o.AIsNull());
        Assert.True(o.BIsNull());
        Assert.True(o.DIsNull());
    }
}