using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// A `&[u8]` param backing an owned-but-borrowing opaque return becomes a
// ReadOnlyMemory<byte> that is pinned for the returned view's whole lifetime
// (PR #1201). These tests exercise the pin: correct reads, GC survival while
// the caller drops every other reference, the error arm's early unpin, and
// disposal.
public class PinnedSliceTests
{
    [Fact]
    public void Parse_RoundTrips_LengthGetSum()
    {
        byte[] data = [1, 2, 3, 4, 5];

        using OpaqueSliceView view = OpaqueSliceView.Parse(data);

        Assert.Equal(5u, view.Length());
        Assert.Equal(3, view.Get(2));
        Assert.Equal(15u, view.Sum());
        Assert.Equal(0, view.Get(99)); // out-of-range is 0, not a throw
    }

    // Build and parse inside an AggressiveOptimization frame so Tier1's precise
    // liveness drops the byte[] local at its last use. The only thing keeping
    // it alive AND pinned afterwards is the view's DiplomatPinnedMemory edge.
    [MethodImpl(MethodImplOptions.NoInlining
#if !NETFRAMEWORK
        | MethodImplOptions.AggressiveOptimization
#endif
    )]
    private static OpaqueSliceView ParseAndDropBuffer()
    {
        byte[] data = [10, 20, 30, 40];
        return OpaqueSliceView.Parse(data);
    }

    [Fact]
    public void ParsedView_KeepsBufferPinnedAndAliveAcrossGc()
    {
        OpaqueSliceView view = ParseAndDropBuffer();

        for (int i = 0; i < 10; i++)
        {
            _ = new byte[256 * 1024];
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        // If the buffer had been collected or moved, these read freed/relocated
        // memory. Sum reads every byte, so it fails hardest on a bad pin.
        Assert.Equal(4u, view.Length());
        Assert.Equal(30, view.Get(2));
        Assert.Equal(100u, view.Sum());
        GC.KeepAlive(view);
    }

    [Fact]
    public void Parse_EmptyBuffer_ThrowsAndSurvives()
    {
        Assert.Throws<SliceParseErrorException>(() => OpaqueSliceView.Parse(Array.Empty<byte>()));

        // The error arm unpinned its buffer before throwing; the runtime is
        // still healthy afterwards.
        using OpaqueSliceView view = OpaqueSliceView.Parse(new byte[] { 7, 8 });
        Assert.Equal(15u, view.Sum());
    }

    // ParseStrict errs on a NON-empty buffer, so a real GCHandle is pinned and
    // must be disposed on the throw path (the empty-buffer case pins nothing).
    // If the catch leaked the handle or disposed a bad one, GC churn afterwards
    // would corrupt the heap; a healthy subsequent Parse proves it didn't.
    [Fact]
    public void ParseStrict_PinnedBufferThatErrors_DisposesPinAndSurvives()
    {
        Assert.Throws<SliceParseErrorException>(
            () => OpaqueSliceView.ParseStrict(new byte[] { 0, 1, 2, 3 }));

        for (int i = 0; i < 10; i++)
        {
            _ = new byte[256 * 1024];
            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        using OpaqueSliceView view = OpaqueSliceView.ParseStrict(new byte[] { 4, 5, 6 });
        Assert.Equal(15u, view.Sum());
    }

    [Fact]
    public void Dispose_MakesCallsThrow_AndDoubleDisposeIsSafe()
    {
        OpaqueSliceView view = OpaqueSliceView.Wrap(new byte[] { 1, 2, 3 });
        Assert.Equal(3u, view.Length());

        view.Dispose();
        Assert.Throws<ObjectDisposedException>(() => view.Length());

        view.Dispose(); // no double-free, no throw
    }

    [Fact]
    public void Parse_SubRange_ReflectsSlicedBuffer()
    {
        byte[] data = [10, 20, 30, 40, 50, 60];

        using OpaqueSliceView view = OpaqueSliceView.Parse(data.AsMemory(2, 3));

        Assert.Equal(3u, view.Length());
        Assert.Equal(30, view.Get(0));
        Assert.Equal(50, view.Get(2));
        Assert.Equal(120u, view.Sum());
    }
}
