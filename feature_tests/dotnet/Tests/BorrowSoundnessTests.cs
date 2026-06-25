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
        // Prove the native pointer (not just the managed wrapper) survived:
        // Get() reads back through the borrowed `&BorrowParent`, so a freed
        // parent would surface as a use-after-free here.
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

    // Regression for the `this`-named-parameter edge case. ViewFrom borrows a
    // Rust param named `this`; the edge must render `@this` (the C# param), not
    // the receiver keyword `this` — bare `this` in this STATIC method wouldn't
    // compile, so this test building at all guards the fix. The GC assertion
    // then confirms the param-edge (not a receiver) roots the parent.
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
