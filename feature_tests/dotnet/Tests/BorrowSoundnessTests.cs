using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class BorrowSoundnessTests
{
    [Fact]
    public void View_WhileParentAlive_GetReturnsParentValue()
    {
        using BorrowParent parent = BorrowParent.Create(42);
        Assert.Equal(42u, parent.Value());
        using BorrowChild child = parent.View();
        Assert.Equal(42u, child.Get());
        GC.KeepAlive(parent);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static (WeakReference parentRef, BorrowChild child) MakeChildAndDropParent()
    {
        BorrowParent parent = BorrowParent.Create(42);
        BorrowChild child = parent.View();
        return (new WeakReference(parent), child);
    }

    [Fact]
    public void Child_KeepsParent_AliveAcrossGc()
    {
        (WeakReference parentRef, BorrowChild child) = MakeChildAndDropParent();
        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();
        Assert.True(
            parentRef.IsAlive,
            "live BorrowChild must keep its borrowed-from BorrowParent alive");
        // Get() reads through the borrow, proving the native pointer survived.
        Assert.Equal(42u, child.Get());
        GC.KeepAlive(child);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static (WeakReference parentRef, BorrowChild child) MakeChildFromThisAndDropParent()
    {
        BorrowParent parent = BorrowParent.Create(7);
        BorrowChild child = BorrowParent.ViewFrom(parent);
        return (new WeakReference(parent), child);
    }

    // Regression: a param named `this` must render `@this`, not the receiver
    // keyword (which wouldn't compile in this static method — so building guards it).
    [Fact]
    public void ViewFrom_ParamNamedThis_KeepsParentAlive()
    {
        (WeakReference parentRef, BorrowChild child) = MakeChildFromThisAndDropParent();
        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();
        Assert.True(parentRef.IsAlive, "param-edge must keep the borrowed-from parent alive");
        Assert.Equal(7u, child.Get());
        GC.KeepAlive(child);
    }
}
