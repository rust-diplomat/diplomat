using System;
using System.Reflection;
using System.Runtime.CompilerServices;
using Somelib;
using Somelib.Diplomat;
using Xunit;

namespace Somelib.FeatureTests;

public class OwnedSliceReturnTests
{
    [Fact]
    public void MakeBytes_ReturnsExpectedContent()
    {
        using RustVec vec = OwnedSliceReturn.MakeBytes(5);

        Assert.Equal(5, vec.Length);
        Assert.Equal(new byte[] { 0, 1, 2, 3, 4 }, vec.Clone());
    }

    [Fact]
    public void MakeBytes_EmptyBuffer_ReturnsEmptyClone()
    {
        using RustVec vec = OwnedSliceReturn.MakeBytes(0);

        Assert.Equal(0, vec.Length);
        Assert.Empty(vec.Clone());
    }

    [Fact]
    public void MakeBytes_LargeBuffer_RoundTripsEveryByte()
    {
        const int len = 4 * 1024 * 1024;
        using RustVec vec = OwnedSliceReturn.MakeBytes(len);

        Assert.Equal(len, vec.Length);
        vec.WithSpan(span =>
        {
            for (int i = 0; i < len; i += 4096)
            {
                Assert.Equal((byte)(i % 256), span[i]);
            }
            Assert.Equal((byte)((len - 1) % 256), span[len - 1]);
        });
    }

    [Fact]
    public void WithSpan_ProvidesZeroCopySynchronousAccess()
    {
        using RustVec vec = OwnedSliceReturn.MakeBytes(64);

        vec.WithSpan(span =>
        {
            Assert.Equal(64, span.Length);
            for (int i = 0; i < 64; i++)
            {
                Assert.Equal((byte)i, span[i]);
            }
        });
    }

    [Fact]
    public void Clone_ReturnsIndependentManagedCopy()
    {
        using RustVec vec = OwnedSliceReturn.MakeBytes(3);

        byte[] clone = vec.Clone();
        clone[0] = 99;

        Assert.Equal(new byte[] { 0, 1, 2 }, vec.Clone());
    }

    [Fact]
    public void WithSpan_MutatesRustAllocationWithoutCopying()
    {
        using RustVec vec = OwnedSliceReturn.MakeBytes(3);

        vec.WithSpan(span => span[0] = 99);

        Assert.Equal(new byte[] { 99, 1, 2 }, vec.Clone());
    }

    [Fact]
    public void RustVec_DoesNotExposeEscapingMemoryView()
    {
        const BindingFlags publicInstance = BindingFlags.Public | BindingFlags.Instance;

        Assert.Null(typeof(RustVec).GetMethod("GetSpan", publicInstance));
        Assert.Null(typeof(RustVec).GetProperty("Memory", publicInstance));
        Assert.Contains(typeof(IDisposable), typeof(RustVec).GetInterfaces());
    }

    [Fact]
    public void Dispose_IsIdempotentAndRejectsFurtherAccess()
    {
        RustVec vec = OwnedSliceReturn.MakeBytes(3);

        vec.Dispose();
        vec.Dispose();

        Assert.Throws<ObjectDisposedException>(() => _ = vec.Length);
        Assert.Throws<ObjectDisposedException>(() => vec.Clone());
        Assert.Throws<ObjectDisposedException>(() => vec.WithSpan(_ => { }));
    }

    [Fact]
    public void Dispose_DuringWithSpanIsRejected()
    {
        using RustVec vec = OwnedSliceReturn.MakeBytes(3);

        vec.WithSpan(span =>
        {
            Assert.Throws<InvalidOperationException>(() => vec.Dispose());
            Assert.Equal((byte)0, span[0]);
        });
    }

    [Fact]
    public void WithSpan_NullActionIsRejected()
    {
        using RustVec vec = OwnedSliceReturn.MakeBytes(3);

        Assert.Throws<ArgumentNullException>(() => vec.WithSpan(null!));
    }

    [Fact]
    public void WithSpan_ExceptionDoesNotBlockLaterDispose()
    {
        RustVec vec = OwnedSliceReturn.MakeBytes(3);

        Assert.Throws<InvalidOperationException>(() =>
            vec.WithSpan(_ => throw new InvalidOperationException()));

        vec.Dispose();
    }

    // Precise liveness must release the local so the finalizer path is observable.
    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static void MakeAndDropWithoutDisposing(int len)
    {
        RustVec vec = OwnedSliceReturn.MakeBytes((uint)len);
        GC.KeepAlive(vec);
    }

    [Fact]
    public void Finalizer_FreesNativeBuffer()
    {
        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();
        long baseline = RustVec.DebugLiveCount;

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
