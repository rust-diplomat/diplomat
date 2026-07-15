using System;
using Raw = Somelib.Raw;

namespace Somelib.Diplomat;

#nullable enable

public delegate void RustVecSpanAction(Span<byte> span);

/// <summary>
/// Owns a Rust <c>Box&lt;[u8]&gt;</c> without copying it into managed memory.
/// This intentionally does not implement <c>MemoryManager&lt;byte&gt;</c> or expose a span:
/// <c>MemoryManager</c> requires <c>GetSpan()</c>, whose result does not keep this owner alive.
/// Use <see cref="WithSpan"/> for synchronous zero-copy access, or <see cref="Clone"/>
/// when an explicit GC-managed copy is needed. Use <c>using</c> for deterministic cleanup;
/// the finalizer is a fallback when the owner is not disposed.
/// </summary>
public sealed class RustVec : IDisposable
{
    private readonly object _gate = new object();
    private IntPtr _ptr;
    private readonly int _length;
    private readonly bool _hasMemoryPressure;
    private int _activeCallbacks;
    private bool _disposed;

    /// <summary>
    /// Test hook for observing that finalization released the native allocation.
    /// </summary>
    internal static long DebugLiveCount;

    internal unsafe RustVec(byte* ptr, nuint length)
    {
        if (length > (nuint)int.MaxValue)
        {
            Raw.RustVec.Destroy(ptr, length);
            GC.SuppressFinalize(this);
            throw new IndexOutOfRangeException("Owned Rust slice is too large for a .NET Span/Memory");
        }
        _ptr = (IntPtr)ptr;
        _length = (int)length;
        System.Threading.Interlocked.Increment(ref DebugLiveCount);
        if (_length > 0)
        {
            GC.AddMemoryPressure(_length);
            _hasMemoryPressure = true;
        }
    }

    public int Length
    {
        get
        {
            lock (_gate)
            {
                ThrowIfDisposed();
                return _length;
            }
        }
    }

    /// <summary>
    /// Provides synchronous zero-copy access. The span is valid only for this callback.
    /// </summary>
    public void WithSpan(RustVecSpanAction action)
    {
        if (action is null)
        {
            throw new ArgumentNullException(nameof(action));
        }
        lock (_gate)
        {
            ThrowIfDisposed();
            _activeCallbacks++;
            try
            {
                unsafe
                {
                    action(_ptr == IntPtr.Zero ? Span<byte>.Empty : new Span<byte>(_ptr.ToPointer(), _length));
                }
            }
            finally
            {
                _activeCallbacks--;
                GC.KeepAlive(this);
            }
        }
    }

    /// <summary>
    /// Explicitly clones the Rust allocation into a GC-managed array.
    /// </summary>
    public byte[] Clone()
    {
        lock (_gate)
        {
            ThrowIfDisposed();
            byte[] clone = new byte[_length];
            try
            {
                if (_ptr != IntPtr.Zero)
                {
                    unsafe
                    {
                        new ReadOnlySpan<byte>(_ptr.ToPointer(), _length).CopyTo(clone);
                    }
                }
            }
            finally
            {
                GC.KeepAlive(this);
            }
            return clone;
        }
    }

    public void Dispose()
    {
        lock (_gate)
        {
            if (_activeCallbacks != 0)
            {
                throw new InvalidOperationException("Cannot dispose RustVec during a WithSpan callback");
            }
            Release();
        }
        GC.SuppressFinalize(this);
    }

    private void ThrowIfDisposed()
    {
        if (_disposed)
        {
            throw new ObjectDisposedException(nameof(RustVec));
        }
    }

    private void Release()
    {
        if (_disposed)
        {
            return;
        }
        _disposed = true;
        IntPtr ptr = _ptr;
        _ptr = IntPtr.Zero;
        if (ptr != IntPtr.Zero)
        {
            unsafe
            {
                Raw.RustVec.Destroy((byte*)ptr, (nuint)_length);
            }
        }
        if (_hasMemoryPressure)
        {
            GC.RemoveMemoryPressure(_length);
        }
        System.Threading.Interlocked.Decrement(ref DebugLiveCount);
    }

    ~RustVec()
    {
        lock (_gate)
        {
            Release();
        }
    }
}