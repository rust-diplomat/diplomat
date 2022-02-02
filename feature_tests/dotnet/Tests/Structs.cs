using System;

using Xunit;

namespace DiplomatFeatures.Tests;

public class StructTests
{
    [Fact]
    public void OpaqueNew()
    {
        Opaque o = Opaque.New();
        MyStruct s = MyStruct.New();
        o.AssertStruct(s);
    }

    [Fact]
    public void MyStructNew()
    {
        MyStruct s = MyStruct.New();
        Assert.Equal(17, s.A);
        Assert.True(s.B);
        Assert.Equal(209, s.C);
        Assert.Equal((ulong)1234, s.D);
        Assert.Equal(5991, s.E);
        Assert.Equal((uint)0x9910, s.F);
    }

    [Fact]
    public void MyStructMutate()
    {
        MyStruct s = MyStruct.New();

        s.A = 0;
        s.B = false;
        s.C = 200;
        s.D = 4321;
        s.E = 1995;
        s.F = 0x9911;

        Assert.Equal(0, s.A);
        Assert.False(s.B);
        Assert.Equal(200, s.C);
        Assert.Equal((ulong)4321, s.D);
        Assert.Equal(1995, s.E);
        Assert.Equal((uint)0x9911, s.F);
    }

    [Fact]
    public void DisposedException()
    {
        Opaque o = Opaque.New();
        MyStruct s = MyStruct.New();

        o.Dispose();
        try
        {
            o.AssertStruct(s);
            Assert.True(false, "expected error didn't occur");
        }
        catch (ObjectDisposedException e)
        {
            Assert.Equal("Opaque", e.ObjectName);
        }
    }
}
