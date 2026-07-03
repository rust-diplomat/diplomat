using System;
using System.Buffers;

namespace Somelib.Diplomat;

#nullable enable

/// <summary>
/// Keeps a caller-provided buffer pinned while a Rust-side value borrows it.
/// The wrapper that borrows the buffer owns this holder (as a keep-alive edge)
/// and disposes it right after running the Rust destructor — never before, so
/// Rust's Drop can still read the buffer. There is deliberately no finalizer:
/// giving the holder its own would let it unpin ahead of the owning wrapper's
/// finalizer in the same GC cycle.
/// </summary>
/// <remarks>
/// This uses <c>System.ReadOnlyMemory&lt;T&gt;</c> and
/// <c>System.Buffers.MemoryHandle</c>. Those are built in on .NET Core 2.1+ /
/// .NET Standard 2.1+, but on .NET Standard 2.0 and .NET Framework they ship in
/// the <c>System.Memory</c> NuGet package — if you target those and use a
/// method that borrows a slice, add a reference to <c>System.Memory</c> or this
/// file will not compile.
/// </remarks>
internal sealed unsafe class DiplomatPinnedMemory : IDisposable
{
    private MemoryHandle _handle;
    private bool _disposed;

    private DiplomatPinnedMemory(MemoryHandle handle)
    {
        _handle = handle;
    }

    public static DiplomatPinnedMemory Pin<T>(ReadOnlyMemory<T> memory)
    {
        // Dispose the handle if wrapping it throws (e.g. OOM), else the buffer
        // would stay pinned with no holder for the caller to unpin.
        MemoryHandle handle = memory.Pin();
        try
        {
            return new DiplomatPinnedMemory(handle);
        }
        catch
        {
            handle.Dispose();
            throw;
        }
    }

    public void* Pointer => _handle.Pointer;

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }
        _disposed = true;
        _handle.Dispose();
    }
}