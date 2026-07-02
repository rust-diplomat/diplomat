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
        return new DiplomatPinnedMemory(memory.Pin());
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