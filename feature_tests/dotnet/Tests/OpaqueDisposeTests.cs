using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class OpaqueDisposeTests
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
    public void DefaultProbe_IsFinalizerOnly_AndDropsExactlyOnce()
    {
        DefaultDropProbe.ResetDropCount();
        Assert.DoesNotContain(typeof(IDisposable), typeof(DefaultDropProbe).GetInterfaces());

        WeakReference weak = CreateDefaultProbeAndDropReference();
        ForceGcUntil(() => !weak.IsAlive && DefaultDropProbe.DropCount() == 1ul);

        Assert.False(weak.IsAlive);
        Assert.Equal(1ul, DefaultDropProbe.DropCount());
    }

    [Fact]
    public void Dispose_DropsNativeOnce()
    {
        DisposableDropProbe.ResetDropCount();
        Assert.Contains(typeof(IDisposable), typeof(DisposableDropProbe).GetInterfaces());

        DisposableDropProbe probe = DisposableDropProbe.Create();
        Assert.True(probe.IsAlive());
        Assert.Equal(0ul, DisposableDropProbe.DropCount());

        probe.Dispose();
        Assert.Equal(1ul, DisposableDropProbe.DropCount());
        Assert.Throws<ObjectDisposedException>(() => probe.IsAlive());

        probe.Dispose();
        Assert.Equal(1ul, DisposableDropProbe.DropCount());
    }

    [Fact]
    public void UsingBlock_DisposesAtScopeExit()
    {
        DisposableDropProbe.ResetDropCount();

        DisposableDropProbe captured;
        using (DisposableDropProbe probe = DisposableDropProbe.Create())
        {
            Assert.True(probe.IsAlive());
            Assert.Equal(0ul, DisposableDropProbe.DropCount());
            captured = probe;
        }

        Assert.Equal(1ul, DisposableDropProbe.DropCount());
        Assert.Throws<ObjectDisposedException>(() => captured.IsAlive());
    }

    [Fact]
    public void DisposeThenFinalizerPass_DoesNotDoubleDrop()
    {
        DisposableDropProbe.ResetDropCount();

        DisposableDropProbe probe = DisposableDropProbe.Create();
        WeakReference weak = new WeakReference(probe);

        probe.Dispose();
        Assert.Equal(1ul, DisposableDropProbe.DropCount());

        probe = null!;
        ForceGcUntil(() => !weak.IsAlive);

        Assert.False(weak.IsAlive);
        Assert.Equal(1ul, DisposableDropProbe.DropCount());
    }
}
