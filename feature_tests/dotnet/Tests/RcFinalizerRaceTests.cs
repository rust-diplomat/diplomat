using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Caller-synchronized lifetime coverage. The no-RC contract does not promise
// safe concurrent call/dispose races, so this test keeps every operation on
// one thread and checks that temporary views release only borrow bookkeeping.
[Collection(RcSharedNativeStateCollection.Name)]
public class RcFinalizerRaceTests
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

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static void ForceGcUntil(Func<bool> condition)
    {
        for (int i = 0; i < 50 && !condition(); i++)
        {
            RcTestGc.DrainFinalizers();
        }
    }

    [Fact]
    public void RepeatedBorrowedViewsReleaseWithoutRetainingNativeOwnership()
    {
        RcTestGc.DrainFinalizers();
        RcSource.ResetDropStats();

        WeakReference sourceRef = CreateSourceAfterRepeatedBorrowedViews();
        ForceGcUntil(() => !sourceRef.IsAlive && RcSource.DropCount() == 1ul);

        Assert.False(sourceRef.IsAlive);
        Assert.Equal(1ul, RcSource.DropCount());
    }
}
