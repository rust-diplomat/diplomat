using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Temporary views release only borrow bookkeeping; they never retain native ownership.
[Collection(RcSharedNativeStateCollection.Name)]
public class RcBorrowReleaseTests
{
    [MethodImpl(MethodImplOptions.NoInlining)]
    private static WeakReference CreateSourceAfterRepeatedBorrowedViews()
    {
        RcSource source = RcSource.Create(42);

        for (int i = 0; i < 1_000; i++)
        {
            RcSource view = source.View();
            Assert.Equal(42ul, view.Id());
        }

        return new WeakReference(source);
    }

    [Fact]
    public void RepeatedBorrowedViewsReleaseWithoutRetainingNativeOwnership()
    {
        TestGc.DrainFinalizers();
        RcSource.ResetDropStats();

        WeakReference sourceRef = CreateSourceAfterRepeatedBorrowedViews();
        TestGc.ForceUntil(() => !sourceRef.IsAlive && RcSource.DropCount() == 1ul);

        Assert.False(sourceRef.IsAlive);
        Assert.Equal(1ul, RcSource.DropCount());
    }
}
