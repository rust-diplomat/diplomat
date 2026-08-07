using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Regression coverage for the pin-lifetime bug: an opaque wrapper's own
// pinned input buffer(s) must only be unpinned once the SHARED
// RustHandleState refcount actually reaches zero — i.e. strictly after this
// wrapper's own Rust destructor runs — even when that destructor call is
// deferred behind a still-outstanding RC dependent rather than triggered by
// this wrapper's own `Dispose()`/finalizer.
//
// Before the fix, an opaque's `Cleanup()` called `_inner.Release()` (which,
// in the deferred case, only decrements the shared refcount without running
// the destructor) and then *unconditionally* disposed its own pinned edges
// right afterwards, regardless of whether the destructor had actually run.
// A later deferred Rust `Drop` could then read an already-unpinned,
// possibly-moved buffer.
//
// `PinnedRcSource` combines both mechanisms in one type (a pinned input AND
// an RC borrow-dependency source), exactly matching the bug's shape. Its
// Rust `Drop` reads the borrowed slice and records a checksum, so a
// moved/corrupted buffer would be directly observable from here.
public class PinLifetimeTests
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
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();
        }
    }

    private static readonly byte[] SourceBytes = { 3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5 };

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
    // source's RustHandleState can keep it rooted/unmovable from here on.
    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static (PinnedRcSource source, PinnedRcDependent dependent, WeakReference bufferRef)
        CreatePinnedPairAndDropBufferReference()
    {
        byte[] buffer = (byte[])SourceBytes.Clone();
        PinnedRcSource source = PinnedRcSource.Create(buffer);
        PinnedRcDependent dependent = source.MakeDependent();
        return (source, dependent, new WeakReference(buffer));
    }

    [Fact]
    public void Source_DisposedWhileDependentLive_DeferDestructorAndKeepsPinAlive()
    {
        PinnedRcSource.ResetDropStats();
        PinnedRcDependent.ResetDropStats();

        (PinnedRcSource source, PinnedRcDependent dependent, WeakReference bufferRef) =
            CreatePinnedPairAndDropBufferReference();

        // Dispose the source first ("outer to inner", the order a caller
        // would naturally use) while the dependent still holds a retained
        // reference. The Rust destructor — and therefore the unpin — must
        // be deferred.
        source.Dispose();
        Assert.Equal(0ul, PinnedRcSource.DropCount());

        // Force a full GC/compaction pass. If the fix is correct, the pin
        // is still held inside the (still-outstanding) RustHandleState, so
        // the buffer must remain alive and reachable via the WeakReference
        // — a deterministic signal, not a probabilistic one. Under the old
        // buggy implementation, the pin would already have been disposed
        // right after `_inner.Release()` regardless of deferral, making the
        // buffer immediately collectible here.
        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();
        Assert.True(
            bufferRef.IsAlive,
            "the source's own pinned input buffer must stay pinned/alive while its Rust " +
            "destructor is deferred behind an outstanding RC dependent"
        );
        Assert.Equal(0ul, PinnedRcSource.DropCount());

        // Disposing the dependent releases the last reference, which must
        // run the source's Rust destructor (reading the still-valid,
        // still-pinned buffer) and only THEN unpin it.
        dependent.Dispose();
        Assert.Equal(1ul, PinnedRcDependent.DropCount());
        Assert.Equal(1ul, PinnedRcSource.DropCount());
        Assert.Equal(ExpectedChecksum(), PinnedRcSource.DropChecksum());

        ulong dependentSeq = PinnedRcDependent.DropSeq();
        ulong sourceSeq = PinnedRcSource.DropSeq();
        Assert.True(dependentSeq != 0 && sourceSeq != 0);
        Assert.True(
            dependentSeq < sourceSeq,
            $"expected dependent (seq {dependentSeq}) to be destroyed before source (seq {sourceSeq})"
        );

        // Finally, the pin must eventually be released (no permanent leak):
        // once nothing references it anymore, the buffer becomes
        // collectible.
        ForceGcUntil(() => !bufferRef.IsAlive);
        Assert.False(
            bufferRef.IsAlive,
            "the pin must be released once the source's destructor has actually run, " +
            "so the buffer eventually becomes collectible"
        );
    }

    [Fact]
    public void Source_DisposedAfterDependent_RunsImmediatelyAndUnpinsAfterDestructor()
    {
        PinnedRcSource.ResetDropStats();
        PinnedRcDependent.ResetDropStats();

        byte[] buffer = (byte[])SourceBytes.Clone();
        PinnedRcSource source = PinnedRcSource.Create(buffer);
        PinnedRcDependent dependent = source.MakeDependent();

        // Dispose "inner to outer" this time: the dependent first, so by
        // the time the source is disposed it is the last reference and its
        // own Release() call is the one that actually reaches zero.
        dependent.Dispose();
        Assert.Equal(1ul, PinnedRcDependent.DropCount());
        Assert.Equal(0ul, PinnedRcSource.DropCount());

        source.Dispose();
        Assert.Equal(1ul, PinnedRcSource.DropCount());
        Assert.Equal(ExpectedChecksum(), PinnedRcSource.DropChecksum());

        GC.KeepAlive(buffer);
    }

    [Fact]
    public void Source_DoubleDispose_UnpinsExactlyOnce_NoMatterHowManyDisposeCalls()
    {
        PinnedRcSource.ResetDropStats();
        PinnedRcDependent.ResetDropStats();

        byte[] buffer = (byte[])SourceBytes.Clone();
        PinnedRcSource source = PinnedRcSource.Create(buffer);
        PinnedRcDependent dependent = source.MakeDependent();

        dependent.Dispose();
        dependent.Dispose();
        source.Dispose();
        source.Dispose();
        source.Dispose();

        Assert.Equal(1ul, PinnedRcDependent.DropCount());
        Assert.Equal(1ul, PinnedRcSource.DropCount());
        Assert.Equal(ExpectedChecksum(), PinnedRcSource.DropChecksum());

        GC.KeepAlive(buffer);
    }

    [Fact]
    public void Source_FinalizedWhileDependentLive_DeferDestructorAndKeepsPinAlive_ViaGc()
    {
        PinnedRcSource.ResetDropStats();
        PinnedRcDependent.ResetDropStats();

        (WeakReference sourceRef, WeakReference dependentRef, WeakReference bufferRef) =
            CreateUnreferencedPinnedPairAndDependent();

        // Neither wrapper is explicitly disposed: both must eventually be
        // collected and finalized. The dependent's finalizer must still run
        // (and release its retained dependency) before the source's own
        // destructor physically runs and unpins the buffer.
        ForceGcUntil(() =>
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

        ulong dependentSeq = PinnedRcDependent.DropSeq();
        ulong sourceSeq = PinnedRcSource.DropSeq();
        Assert.True(dependentSeq != 0 && sourceSeq != 0);
        Assert.True(
            dependentSeq < sourceSeq,
            $"expected finalizer-only dependent (seq {dependentSeq}) to be destroyed before source (seq {sourceSeq})"
        );

        ForceGcUntil(() => !bufferRef.IsAlive);
        Assert.False(bufferRef.IsAlive, "the pin must eventually be released after both finalizers ran");
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
