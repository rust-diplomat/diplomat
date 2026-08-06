using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class IdisposableOptInTests
{
    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static WeakReference CreateDefaultProbeAndDropReference()
    {
        DefaultDropProbe probe = DefaultDropProbe.Create();
        return new WeakReference(probe);
    }

    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static void ForceGcUntil(Func<bool> condition)
    {
        for (int i = 0; i < 50 && !condition(); i++)
        {
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();
        }
    }

    [Fact]
    public void DefaultProbe_UsesFinalizerOnly_AndDropsExactlyOnce()
    {
        DefaultDropProbe.ResetDropCount();
        Assert.DoesNotContain(typeof(IDisposable), typeof(DefaultDropProbe).GetInterfaces());

        WeakReference weak = CreateDefaultProbeAndDropReference();
        ForceGcUntil(() => !weak.IsAlive && DefaultDropProbe.DropCount() == 1ul);

        Assert.False(weak.IsAlive);
        Assert.Equal(1ul, DefaultDropProbe.DropCount());
    }

    [Fact]
    public void OptInProbe_DisposeDropsSynchronously_AndNoDoubleDropAfterFinalizerPass()
    {
        DisposableDropProbe.ResetDropCount();
        Assert.Contains(typeof(IDisposable), typeof(DisposableDropProbe).GetInterfaces());

        DisposableDropProbe probe = DisposableDropProbe.Create();
        WeakReference weak = new WeakReference(probe);

        probe.Dispose();
        Assert.Equal(1ul, DisposableDropProbe.DropCount());

        probe.Dispose();
        Assert.Equal(1ul, DisposableDropProbe.DropCount());

        probe = null!;
        ForceGcUntil(() => !weak.IsAlive);

        Assert.False(weak.IsAlive);
        Assert.Equal(1ul, DisposableDropProbe.DropCount());
    }
}
