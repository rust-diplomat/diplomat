using System;
using System.Runtime.CompilerServices;
using Xunit;

namespace Somelib.FeatureTests;

// The native drop counters are shared across tests.
[CollectionDefinition(Name, DisableParallelization = true)]
public class RcSharedNativeStateCollection
{
    public const string Name = "RcSharedNativeState";
}

internal static class TestGc
{
    [MethodImpl(MethodImplOptions.NoInlining)]
    internal static void DrainFinalizers()
    {
        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();
    }

    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    internal static void ForceUntil(Func<bool> condition)
    {
        for (int i = 0; i < 50 && !condition(); i++)
        {
            DrainFinalizers();
        }
    }
}
