using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

[Collection(RcSharedNativeStateCollection.Name)]
public class PinLifetimeTests
{
    private static readonly byte[] SourceBytes = { 3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5 };

    private static void ResetPinnedDropStats()
    {
        TestGc.DrainFinalizers();
        PinnedRcSource.ResetDropStats();
        PinnedRcDependent.ResetDropStats();
    }

    private static ulong ExpectedChecksum()
    {
        ulong checksum = 0;
        foreach (byte b in SourceBytes)
        {
            checksum += b;
        }
        return checksum;
    }

    // Builds the source/dependent pair from a freshly-allocated buffer and
    // returns a WeakReference to that buffer without holding any other
    // strong managed reference to it. Only the (still-live) pin inside the
    // source's RustHandle can keep it rooted/unmovable from here on.
    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static (WeakReference sourceRef, PinnedRcDependent dependent, WeakReference bufferRef)
        CreatePinnedPairAndDropBufferReference()
    {
        byte[] buffer = (byte[])SourceBytes.Clone();
        PinnedRcSource source = PinnedRcSource.Create(buffer);
        PinnedRcDependent dependent = source.MakeDependent();
        return (new WeakReference(source), dependent, new WeakReference(buffer));
    }

    [Fact]
    public void DependentDispose_ReleasesSourcePinAndNativeState()
    {
        ResetPinnedDropStats();

        (WeakReference sourceRef, PinnedRcDependent dependent, WeakReference bufferRef) =
            CreatePinnedPairAndDropBufferReference();

        Assert.Equal(0ul, PinnedRcSource.DropCount());
        Assert.True(bufferRef.IsAlive);

        dependent.Dispose();
        TestGc.ForceUntil(() =>
            !sourceRef.IsAlive
            && !bufferRef.IsAlive
            && PinnedRcSource.DropCount() == 1ul
            && PinnedRcDependent.DropCount() == 1ul
        );

        Assert.Equal(1ul, PinnedRcSource.DropCount());
        Assert.Equal(ExpectedChecksum(), PinnedRcSource.DropChecksum());
        Assert.Equal(1ul, PinnedRcDependent.DropCount());
        Assert.False(bufferRef.IsAlive);
    }

    [Fact]
    public void DependentDispose_IsIdempotentThenSourceFinalizesOnce()
    {
        ResetPinnedDropStats();

        WeakReference sourceRef = CreateDisposedDependentAndDropSourceReference();

        Assert.Equal(1ul, PinnedRcDependent.DropCount());
        TestGc.ForceUntil(() => !sourceRef.IsAlive && PinnedRcSource.DropCount() == 1ul);

        Assert.Equal(1ul, PinnedRcSource.DropCount());
        Assert.Equal(ExpectedChecksum(), PinnedRcSource.DropChecksum());
    }

    [Fact]
    public void UnreferencedPinnedPair_EventuallyCleansUpAndReleasesPin()
    {
        ResetPinnedDropStats();

        (WeakReference sourceRef, WeakReference dependentRef, WeakReference bufferRef) =
            CreateUnreferencedPinnedPairAndDependent();

        TestGc.ForceUntil(() =>
            !sourceRef.IsAlive
            && !dependentRef.IsAlive
            && PinnedRcSource.DropCount() == 1ul
            && PinnedRcDependent.DropCount() == 1ul
        );

        Assert.False(sourceRef.IsAlive);
        Assert.False(dependentRef.IsAlive);
        Assert.Equal(1ul, PinnedRcSource.DropCount());
        Assert.Equal(1ul, PinnedRcDependent.DropCount());
        Assert.Equal(ExpectedChecksum(), PinnedRcSource.DropChecksum());

        TestGc.ForceUntil(() => !bufferRef.IsAlive);
        Assert.False(bufferRef.IsAlive, "the pin must eventually be released after both finalizers ran");
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static WeakReference CreateDisposedDependentAndDropSourceReference()
    {
        byte[] buffer = (byte[])SourceBytes.Clone();
        PinnedRcSource source = PinnedRcSource.Create(buffer);
        PinnedRcDependent dependent = source.MakeDependent();

        dependent.Dispose();
        dependent.Dispose();

        Assert.Equal(1ul, PinnedRcDependent.DropCount());
        Assert.Equal(0ul, PinnedRcSource.DropCount());

        WeakReference sourceRef = new WeakReference(source);
        GC.KeepAlive(source);
        return sourceRef;
    }

    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static (WeakReference sourceRef, WeakReference dependentRef, WeakReference bufferRef)
        CreateUnreferencedPinnedPairAndDependent()
    {
        byte[] buffer = (byte[])SourceBytes.Clone();
        PinnedRcSource source = PinnedRcSource.Create(buffer);
        PinnedRcDependent dependent = source.MakeDependent();
        return (new WeakReference(source), new WeakReference(dependent), new WeakReference(buffer));
    }
}
