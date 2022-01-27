using System;

using Xunit;

namespace DiplomatFeatures.Tests;

public class OwnershipTests
{
    [Fact]
    public void CounterIsCounting()
    {
        var counter = Counter.New();
        Assert.Equal((nuint)0, counter.Count());
        var counted1 = CountedOpaque.New(counter);
        Assert.Equal((nuint)1, counter.Count());
        var counted2 = CountedOpaque.New(counter);
        Assert.Equal((nuint)2, counter.Count());
        var counted3 = CountedOpaque.New(counter);
        Assert.Equal((nuint)3, counter.Count());
        counted1.Dispose();
        Assert.Equal((nuint)2, counter.Count());
        counted2.Dispose();
        Assert.Equal((nuint)1, counter.Count());
        counted3.Dispose();
        Assert.Equal((nuint)0, counter.Count());
    }

    // FIXME: uncomment when C++ backend support this pattern too
    // Alternatively, this could be uncommented once issue #127 is addressed.

    /* [Fact]
    public void TakeOwnershipByMove()
    {
        var counter = Counter.New();
        Assert.Equal((nuint)0, counter.Count());

        var eater = OwnershipEater.New();
        var counted = CountedOpaque.New(counter);
        Assert.Equal((nuint)1, counter.Count());

        // Move `counted` inside `eater`
        eater.TakeOpaque(counted);
        // Do nothing because `counted` was moved above
        counted.Dispose();
        // Total count should be unchanged.
        Assert.Equal((nuint)1, counter.Count());

        // However, it should be decreased when replaced with nothing
        eater.TakeOpaqueOpt(null);
        Assert.Equal((nuint)0, counter.Count());

        // When eater is dropped, total count decreases as well
        eater.TakeOpaqueOpt(CountedOpaque.New(counter));
        Assert.Equal((nuint)1, counter.Count());
        eater.Dispose();
        Assert.Equal((nuint)0, counter.Count());
    }

    [Fact]
    public void ErrorOnMovedObjectReuse()
    {
        var eater = OwnershipEater.New();
        var counted = CountedOpaque.New(Counter.New());

        eater.TakeOpaque(counted); // counted is moved

        try
        {
            eater.TakeOpaque(counted); // throws because counted is used after having being moved
            Assert.True(false, "expected error didn't occur");
        }
        catch (ObjectDisposedException e)
        {
            Assert.Equal("CountedOpaque", e.ObjectName);
        }
    } */
}
