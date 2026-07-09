using System;
using System.Buffers;
using System.Runtime.CompilerServices;
using Somelib;
using Somelib.Diplomat;
using Xunit;

namespace Somelib.FeatureTests;

// `OwnedSliceReturn.MakeBytes` returns an owned `Box<[u8]>` — on .NET this
// lowers to a zero-copy `RustVec` (`System.Buffers.MemoryManager<byte>`)
// wrapping the raw `(ptr, len)` pair directly, rather than copying into a
// managed `byte[]`. These tests exercise: correct bytes back (small, empty,
// and large buffers), `GetSpan`/`Memory` content after the raw call returns,
// `Dispose()` idempotency, and the finalizer path actually freeing the
// native buffer when `Dispose()` is never called.
public class OwnedSliceReturnTests
{
    [Fact]
    public void MakeBytes_ReturnsExpectedContent()
    {
        using RustVec vec = OwnedSliceReturn.MakeBytes(5);

        Assert.Equal(5, vec.Memory.Length);
        Assert.Equal(new byte[] { 0, 1, 2, 3, 4 }, vec.Memory.ToArray());
    }

    [Fact]
    public void MakeBytes_EmptyBuffer_ReturnsEmptyMemory()
    {
        using RustVec vec = OwnedSliceReturn.MakeBytes(0);

        Assert.Equal(0, vec.Memory.Length);
        Assert.Empty(vec.GetSpan().ToArray());
    }

    // Large enough that a copy-based implementation would be an obviously
    // expensive round trip, and big enough to make GC.AddMemoryPressure's
    // effect meaningful rather than a rounding error.
    [Fact]
    public void MakeBytes_LargeBuffer_RoundTripsEveryByte()
    {
        const int len = 4 * 1024 * 1024;
        using RustVec vec = OwnedSliceReturn.MakeBytes(len);

        Assert.Equal(len, vec.Memory.Length);
        ReadOnlySpan<byte> span = vec.GetSpan();
        for (int i = 0; i < len; i += 4096)
        {
            Assert.Equal((byte)(i % 256), span[i]);
        }
        Assert.Equal((byte)((len - 1) % 256), span[len - 1]);
    }

    // Catches marshaling/offset bugs in the raw DiplomatOwnedSliceU8 struct:
    // GetSpan/Memory must reflect the actual raw call result, not a stale or
    // shifted view of it.
    [Fact]
    public void GetSpan_And_Memory_AgreeWithEachOtherAfterRawCall()
    {
        using RustVec vec = OwnedSliceReturn.MakeBytes(64);

        ReadOnlySpan<byte> span = vec.GetSpan();
        ReadOnlyMemory<byte> memory = vec.Memory;

        Assert.Equal(64, span.Length);
        Assert.Equal(64, memory.Length);
        Assert.True(span.SequenceEqual(memory.Span));
        for (int i = 0; i < 64; i++)
        {
            Assert.Equal((byte)i, span[i]);
        }
    }

    [Fact]
    public void Dispose_IsIdempotent()
    {
        // MemoryManager<T> exposes Dispose() only through IDisposable, not as
        // a public member of RustVec directly — same as any `using (memoryManager)`.
        IDisposable vec = OwnedSliceReturn.MakeBytes(8);

        vec.Dispose();
        vec.Dispose(); // must not throw, must not double-free
    }

    [Fact]
    public void Disposed_Memory_ThrowsObjectDisposedException()
    {
        RustVec vec = OwnedSliceReturn.MakeBytes(8);
        ((IDisposable)vec).Dispose();

        Assert.Throws<ObjectDisposedException>(() => vec.Memory);
    }

    // Builds inside an AggressiveOptimization frame so Tier1's precise
    // liveness drops the local at its last use (same technique as
    // GcRaceTests/PinnedSliceTests) — nothing keeps the RustVec reachable
    // once this method returns.
    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.AggressiveOptimization)]
    private static void MakeAndDropWithoutDisposing(int len)
    {
        RustVec vec = OwnedSliceReturn.MakeBytes((uint)len);
        GC.KeepAlive(vec);
    }

    // The single most valuable test here: proves the finalizer path actually
    // frees the native buffer when Dispose() is never called by the caller.
    // RustVec.DebugLiveCount is a same-assembly test hook (constructed
    // instances increment it, a real free — from either Dispose or the
    // finalizer — decrements it); there is no other way to observe from C#
    // that a *finalized* instance's native memory was freed, since touching
    // the pointer after that would itself be a use-after-free.
    [Fact]
    public void Finalizer_FreesNativeBufferWhenDisposeIsNeverCalled()
    {
        long baseline = RustVec.DebugLiveCount;

        for (int i = 0; i < 200 && RustVec.DebugLiveCount > baseline; i++)
        {
            // Drain stragglers from earlier tests so what we observe below is
            // caused by THIS loop's allocations, not leftover garbage.
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }
        baseline = RustVec.DebugLiveCount;

        for (int i = 0; i < 64; i++)
        {
            MakeAndDropWithoutDisposing(1024);
        }

        Assert.True(
            RustVec.DebugLiveCount > baseline,
            "expected undisposed RustVecs to still be pending finalization before a GC pass");

        bool freed = false;
        for (int attempt = 0; attempt < 50 && !freed; attempt++)
        {
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();
            freed = RustVec.DebugLiveCount == baseline;
        }

        Assert.Equal(baseline, RustVec.DebugLiveCount);
    }
}
