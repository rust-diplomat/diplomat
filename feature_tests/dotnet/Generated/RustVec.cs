using System;
using System.Buffers;
using System.Runtime.InteropServices;

namespace Somelib.Diplomat;

#nullable enable

/// <summary>
/// A zero-copy view over an owned <c>Box&lt;[u8]&gt;</c> Rust handed back across FFI.
/// Wraps the raw <c>(ptr, len)</c> pair directly instead of copying into a managed
/// <c>byte[]</c>. Call <see cref="MemoryManager{T}.Dispose()"/> for deterministic
/// cleanup, or let the finalizer free it later — either way the native buffer is
/// freed exactly once, on the same allocator that made it.
/// </summary>
public sealed unsafe class RustVec : MemoryManager<byte>
{
    [DllImport(DiplomatNativeLib.Name, EntryPoint = "diplomat_owned_slice_u8_destroy", CallingConvention = CallingConvention.Cdecl)]
    private static extern void diplomat_owned_slice_u8_destroy(byte* ptr, nuint len);

    private byte* _ptr;
    private readonly int _length;
    // int, not bool: two racing Dispose calls (user thread vs. user thread,
    // or user thread vs. finalizer) must not both reach the destroy extern —
    // the flag is claimed once via Interlocked.Exchange.
    private int _disposed;

    /// <summary>
    /// Count of constructed-but-not-yet-freed instances. Same assembly as the
    /// generated tests, so this doubles as their leak/finalizer-reachability
    /// probe — there is no other way to observe from C# that the native
    /// buffer behind a finalized instance was actually freed.
    /// </summary>
    internal static long DebugLiveCount;

    internal RustVec(byte* ptr, nuint length)
    {
        if (length > (nuint)int.MaxValue)
        {
            // Free the buffer Rust just handed us before bailing, and suppress
            // the finalizer: it runs even on partially-constructed objects and
            // would otherwise decrement DebugLiveCount without a matching
            // increment.
            diplomat_owned_slice_u8_destroy(ptr, length);
            GC.SuppressFinalize(this);
            throw new IndexOutOfRangeException("Owned Rust slice is too large for a .NET Span/Memory");
        }
        _ptr = ptr;
        _length = (int)length;
        System.Threading.Interlocked.Increment(ref DebugLiveCount);
        // The GC can't see the unmanaged bytes behind this small managed object —
        // without this, a pile of undisposed RustVecs looks free to the GC and it
        // won't collect them promptly even under real memory pressure.
        // GC.AddMemoryPressure requires a strictly positive value — an empty
        // buffer has nothing to account for.
        if (_length > 0)
        {
            GC.AddMemoryPressure(_length);
        }
    }

    public override Span<byte> GetSpan()
    {
        // Local copy: a racing Dispose could null _ptr between the disposed
        // check and the Span construction, and a (null, len > 0) span would
        // AV on first read instead of throwing.
        byte* ptr = _ptr;
        if (_disposed != 0)
        {
            throw new ObjectDisposedException(nameof(RustVec));
        }
        return ptr == null ? Span<byte>.Empty : new Span<byte>(ptr, _length);
    }

    public override MemoryHandle Pin(int elementIndex = 0)
    {
        if (_disposed != 0)
        {
            throw new ObjectDisposedException(nameof(RustVec));
        }
        if ((uint)elementIndex > (uint)_length)
        {
            throw new ArgumentOutOfRangeException(nameof(elementIndex));
        }
        // The buffer is unmanaged and already fixed in place — no GCHandle to
        // allocate. Passing `this` as the IPinnable makes the MemoryHandle keep
        // this manager reachable, so the finalizer can't free the buffer while
        // a caller still holds the pinned pointer.
        return _ptr == null ? default : new MemoryHandle(_ptr + elementIndex, default, this);
    }

    public override void Unpin()
    {
        // No-op: Pin() never allocated a GCHandle, so there's nothing to release.
    }

    protected override void Dispose(bool disposing)
    {
        if (System.Threading.Interlocked.Exchange(ref _disposed, 1) != 0)
        {
            return;
        }
        if (_ptr != null)
        {
            diplomat_owned_slice_u8_destroy(_ptr, (nuint)_length);
            _ptr = null;
        }
        if (_length > 0)
        {
            GC.RemoveMemoryPressure(_length);
        }
        System.Threading.Interlocked.Decrement(ref DebugLiveCount);
    }

    // CA2015 warns that a finalizer on a MemoryManager<T> could free memory
    // out from under a live Span<T> — that's exactly the failure mode
    // GetSpan/Pin's `_disposed` check above exists to turn into an
    // ObjectDisposedException instead of a silent use-after-free, and a
    // finalizer safety net for callers who never call Dispose() is a
    // required part of this type's design (see DECISIONS.md).
#pragma warning disable CA2015
    ~RustVec()
    {
        Dispose(false);
    }
#pragma warning restore CA2015
}