using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Borrow-soundness: BorrowParent.View() returns a BorrowChild that borrows the
// parent (Box<BorrowChild<'a>>). A live child must keep its parent alive so the
// parent is not collected/finalized (-> Destroy) while the child still reads it.
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
        GC.KeepAlive(child);
    }
}
