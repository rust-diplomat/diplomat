using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// The native drop counters are shared across tests.
[CollectionDefinition(Name, DisableParallelization = true)]
public class RcSharedNativeStateCollection
{
    public const string Name = "RcSharedNativeState";
}

internal static class RcTestGc
{
    [MethodImpl(MethodImplOptions.NoInlining)]
    internal static void DrainFinalizers()
    {
        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();
    }
}

[Collection(RcSharedNativeStateCollection.Name)]
public class RcBorrowDependencyTests
{
    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static void ForceGcUntil(Func<bool> condition)
    {
        for (int i = 0; i < 50 && !condition(); i++)
        {
            RcTestGc.DrainFinalizers();
        }
    }

    private static void ResetAllDropStats()
    {
        RcTestGc.DrainFinalizers();
        RcSource.ResetDropStats();
        RcDependent.ResetDropStats();
        RcDependent2.ResetDropStats();
        RcFinalizerSource.ResetDropStats();
        RcFinalizerDependent.ResetDropStats();
    }

    [Fact]
    public void BorrowedView_RemainsUsableWhileSourceIsReachable()
    {
        ResetAllDropStats();

        RcSource source = RcSource.Create(42);
        RcSource view = source.View();

        Assert.Equal(42ul, source.Id());
        Assert.Equal(42ul, view.Id());
        Assert.Equal(0ul, RcSource.DropCount());
        GC.KeepAlive(view);
    }

    [Fact]
    public void RetainedBorrowSources_AreFinalizerOnly()
    {
        Assert.DoesNotContain(typeof(IDisposable), typeof(RcSource).GetInterfaces());
        Assert.DoesNotContain(typeof(IDisposable), typeof(RcDependent).GetInterfaces());
    }

    [Fact]
    public void LiveChildKeepsSourceNativeStateAfterSourceWrapperCollection()
    {
        ResetAllDropStats();

        (WeakReference sourceRef, WeakReference dependentRef) =
            CreateDependentAndVerifySourceReachability();

        Assert.False(sourceRef.IsAlive);

        ForceGcUntil(() =>
            !dependentRef.IsAlive
            && RcSource.DropCount() == 1ul
            && RcDependent.DropCount() == 1ul
        );
        Assert.Equal(1ul, RcSource.DropCount());
        Assert.Equal(1ul, RcDependent.DropCount());
    }

    [Fact]
    public void TransitiveChain_EventuallyCleansEveryResource()
    {
        ResetAllDropStats();

        (WeakReference sourceRef, WeakReference dependentRef, WeakReference dependent2Ref) =
            CreateTransitivePairAndDropReferences();
        ForceGcUntil(() =>
            !sourceRef.IsAlive
            && !dependentRef.IsAlive
            && !dependent2Ref.IsAlive
            && RcSource.DropCount() == 1ul
            && RcDependent.DropCount() == 1ul
            && RcDependent2.DropCount() == 1ul
        );

        Assert.False(sourceRef.IsAlive);
        Assert.False(dependentRef.IsAlive);
        Assert.False(dependent2Ref.IsAlive);
        Assert.Equal(1ul, RcSource.DropCount());
        Assert.Equal(1ul, RcDependent.DropCount());
        Assert.Equal(1ul, RcDependent2.DropCount());
    }

    [Fact]
    public void DisposableChild_CanBeDisposedWithoutDisposableSource()
    {
        ResetAllDropStats();

        RcSource source = RcSource.Create(9);
        RcDependent dependent = source.MakeDependent();
        RcDependent2 dependent2 = dependent.MakeDependent2();

        dependent2.Dispose();
        dependent2.Dispose();

        Assert.Equal(9ul, dependent.Id());
        Assert.Equal(1ul, RcDependent2.DropCount());
        Assert.Equal(0ul, RcSource.DropCount());
        GC.KeepAlive(dependent);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static (WeakReference sourceRef, RcDependent dependent) CreateDependentAndDropSourceReference()
    {
        RcSource source = RcSource.Create(42);
        RcDependent dependent = source.MakeDependent();
        return (new WeakReference(source), dependent);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static (WeakReference sourceRef, WeakReference dependentRef)
        CreateDependentAndVerifySourceReachability()
    {
        (WeakReference sourceRef, RcDependent dependent) =
            CreateDependentAndDropSourceReference();
        ForceGcUntil(() => !sourceRef.IsAlive);

        Assert.False(sourceRef.IsAlive);
        Assert.Equal(42ul, dependent.SourceId());
        Assert.Equal(0ul, RcSource.DropCount());

        WeakReference dependentRef = new WeakReference(dependent);
        GC.KeepAlive(dependent);
        return (sourceRef, dependentRef);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static (WeakReference sourceRef, WeakReference dependentRef, WeakReference dependent2Ref)
        CreateTransitivePairAndDropReferences()
    {
        RcSource source = RcSource.Create(100);
        RcDependent dependent = source.MakeDependent();
        RcDependent2 dependent2 = dependent.MakeDependent2();
        return (
            new WeakReference(source),
            new WeakReference(dependent),
            new WeakReference(dependent2)
        );
    }

    [Fact]
    public void FinalizerOnlyProbes_AreNotIDisposable()
    {
        Assert.DoesNotContain(typeof(IDisposable), typeof(RcSource).GetInterfaces());
        Assert.DoesNotContain(typeof(IDisposable), typeof(RcDependent).GetInterfaces());
        Assert.DoesNotContain(typeof(IDisposable), typeof(RcFinalizerSource).GetInterfaces());
        Assert.DoesNotContain(typeof(IDisposable), typeof(RcFinalizerDependent).GetInterfaces());
    }

    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static (WeakReference sourceRef, WeakReference dependentRef) CreateFinalizerPairAndDropReferences()
    {
        RcFinalizerSource source = RcFinalizerSource.Create(55);
        RcFinalizerDependent dependent = source.MakeDependent();
        return (new WeakReference(source), new WeakReference(dependent));
    }

    [Fact]
    public void FinalizerOnlyPair_EventuallyCleansUpWithoutOrderAssertion()
    {
        ResetAllDropStats();

        (WeakReference sourceRef, WeakReference dependentRef) = CreateFinalizerPairAndDropReferences();

        ForceGcUntil(() =>
            !sourceRef.IsAlive
            && !dependentRef.IsAlive
            && RcFinalizerSource.DropCount() == 1ul
            && RcFinalizerDependent.DropCount() == 1ul
        );

        Assert.False(sourceRef.IsAlive);
        Assert.False(dependentRef.IsAlive);
        Assert.Equal(1ul, RcFinalizerSource.DropCount());
        Assert.Equal(1ul, RcFinalizerDependent.DropCount());
    }
}
