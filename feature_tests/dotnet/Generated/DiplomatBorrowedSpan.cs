using System;
using System.Threading;

namespace Somelib.Diplomat;

#nullable enable

public delegate void DiplomatBorrowedSpanAction<T>(ReadOnlySpan<T> span) where T : unmanaged;

/// <summary>
/// A zero-copy view over memory Rust still owns (a borrowed <c>&amp;str</c> /
/// <c>&amp;[T]</c> return). Unlike <c>RustVec</c>, this object does not own the
/// native bytes. Its source edges keep the managed handles reachable and check
/// source validity before each raw access.
/// </summary>
/// <remarks>
/// This intentionally does not expose a public <c>Span</c>-returning
/// property. A caller could extract it and outlive the source check. Use
/// <see cref="WithSpan"/> for scoped zero-copy access, or <see cref="Clone"/>
/// for an independent managed copy.
/// </remarks>
public sealed unsafe class DiplomatBorrowedSpan<T> : IDisposable where T : unmanaged
{
    private readonly T* _ptr;
    private readonly int _len;
    private ILifetimeEdge?[] _edges = Array.Empty<ILifetimeEdge?>();
    private int _disposed;

    internal DiplomatBorrowedSpan(T* ptr, nuint len, ILifetimeEdge?[] edges)
    {
        if (len > (nuint)int.MaxValue)
        {
            LifetimeEdges.ReleaseNoThrow(edges);
            throw new IndexOutOfRangeException(
                "Borrowed Rust slice is too large for a .NET Span/Memory");
        }

        _ptr = ptr;
        _len = (int)len;
        _edges = edges;
        try
        {
            for (int i = 0; i < _edges.Length; i++)
            {
                if (_edges[i] is IBorrowLease lease)
                {
                    _edges[i] = lease.IntoVersionedEdge();
                }
            }
        }
        catch
        {
            Cleanup();
            throw;
        }
    }

    public int Length => _len;

    /// <summary>
    /// Synchronous, zero-copy, read-only access. The span is valid only for
    /// the duration of this callback.
    /// </summary>
    public void WithSpan(DiplomatBorrowedSpanAction<T> action)
    {
        if (action is null)
        {
            throw new ArgumentNullException(nameof(action));
        }
        ILifetimeEdge[] leased = GetDependenciesForAccess().LeaseForAccess();
        try
        {
            action(new ReadOnlySpan<T>(_ptr, _len));
        }
        finally
        {
            try
            {
                LifetimeEdges.ReleaseLeases(leased);
            }
            finally
            {
                GC.KeepAlive(this);
            }
        }
    }

    /// <summary>An explicit, independent copy — never implicit.</summary>
    public T[] Clone()
    {
        GetDependenciesForAccess().Validate();
        try
        {
            return new ReadOnlySpan<T>(_ptr, _len).ToArray();
        }
        finally
        {
            GC.KeepAlive(this);
        }
    }

    public void Dispose()
    {
        Cleanup();
        GC.SuppressFinalize(this);
    }

    ~DiplomatBorrowedSpan()
    {
        try
        {
            Cleanup();
        }
        catch
        {
        }
    }

    private LifetimeEdges GetDependenciesForAccess()
    {
        if (Volatile.Read(ref _disposed) != 0)
        {
            throw new ObjectDisposedException(nameof(DiplomatBorrowedSpan<T>));
        }

        return new LifetimeEdges(WrapperKind.SharedView, Volatile.Read(ref _edges));
    }

    private void Cleanup()
    {
        if (Interlocked.Exchange(ref _disposed, 1) != 0)
        {
            return;
        }

        ILifetimeEdge?[] edges = Interlocked.Exchange(
            ref _edges,
            Array.Empty<ILifetimeEdge?>());
        LifetimeEdges.ReleaseNoThrow(edges);
    }
}