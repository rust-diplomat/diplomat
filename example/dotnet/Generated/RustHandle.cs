using System;
using System.Collections.Generic;
using System.Threading;

namespace Somelib.Diplomat;

#nullable enable

internal unsafe delegate void RustDestructor<T>(T* ptr) where T : unmanaged;

internal interface ILifetimeEdge
{
    void Release();
}

internal static class LifetimeEdge
{
    internal static ILifetimeEdge? Move<T>(ref BorrowLease<T>? lease) where T : unmanaged
    {
        BorrowLease<T>? moved = Interlocked.Exchange(ref lease, null);
        moved?.TransferToPersistent();
        return moved;
    }
}

internal enum BorrowKind
{
    Shared,
    Exclusive,
}

internal enum WrapperKind
{
    Owned,
    SharedView,
    ExclusiveView,
}

internal readonly struct MutationVersion
{
    private readonly long _value;

    internal MutationVersion(long value)
    {
        _value = value;
    }

    internal bool Equals(MutationVersion other) => _value == other._value;
}

internal struct LifetimeEdges
{
    private ILifetimeEdge?[] _edges;

    internal LifetimeEdges(WrapperKind kind, ILifetimeEdge?[] edges)
    {
        _edges = edges;
        for (int i = 0; i < _edges.Length; i++)
        {
            if (_edges[i] is IBorrowLease lease && IsReadView(kind, lease))
            {
                _edges[i] = lease.IntoVersionedEdge();
            }
        }
    }

    // A value born from an exclusive borrow is its source's only writer until released, so
    // it keeps the borrow; every other dependent is a read view that only remembers a version.
    private static bool IsReadView(WrapperKind kind, IBorrowLease lease) =>
        kind == WrapperKind.SharedView
        || (kind == WrapperKind.Owned && lease.Kind == BorrowKind.Shared);

    internal ILifetimeEdge[] AcquireOperationLeases()
    {
        List<ILifetimeEdge>? acquired = null;
        try
        {
            foreach (ILifetimeEdge? edge in Volatile.Read(ref _edges))
            {
                ILifetimeEdge? operation = edge switch
                {
                    IVersionedReference versioned => versioned.Lease(),
                    IBorrowLease borrow => borrow.AcquireOperation(),
                    _ => null,
                };
                if (operation is not null)
                {
                    (acquired ??= new List<ILifetimeEdge>()).Add(operation);
                }
            }

            return acquired?.ToArray() ?? Array.Empty<ILifetimeEdge>();
        }
        catch
        {
            if (acquired is not null)
            {
                ReleaseNoThrow(acquired.ToArray());
            }
            throw;
        }
    }

    internal void Release()
    {
        ILifetimeEdge?[] edges = Interlocked.Exchange(
            ref _edges,
            Array.Empty<ILifetimeEdge?>());
        ReleaseNoThrow(edges);
    }

    internal static void ReleaseLeases(ILifetimeEdge[] leases)
    {
        for (int i = leases.Length - 1; i >= 0; i--)
        {
            leases[i].Release();
        }
    }

    internal static void ReleaseNoThrow(ILifetimeEdge?[] edges)
    {
        for (int i = edges.Length - 1; i >= 0; i--)
        {
            try
            {
                edges[i]?.Release();
            }
            catch
            {
            }
        }
    }
}

internal struct BorrowLedger
{
    private const int Free = 0;
    private const int Exclusive = -1;

    private int _state;
    private MutationClock _mutations;
    private int _scopeEnded;

    internal bool IsScopeOpen() => Volatile.Read(ref _scopeEnded) == 0;

    internal bool IsCurrent(MutationVersion mutationVersion) =>
        IsScopeOpen() && _mutations.Read().Equals(mutationVersion);

    internal bool EndScope() => Interlocked.Exchange(ref _scopeEnded, 1) == 0;

    internal void Enter(BorrowKind kind)
    {
        if (kind == BorrowKind.Exclusive)
        {
            if (Interlocked.CompareExchange(ref _state, Exclusive, Free) != Free)
            {
                throw new InvalidOperationException("Another borrow is already active.");
            }

            return;
        }

        while (true)
        {
            int current = Volatile.Read(ref _state);
            if (current < Free)
            {
                throw new InvalidOperationException("An exclusive borrow is already active.");
            }

            if (Interlocked.CompareExchange(ref _state, current + 1, current) == current)
            {
                return;
            }
        }
    }

    internal MutationVersion Exit(BorrowKind kind)
    {
        if (kind == BorrowKind.Shared)
        {
            MutationVersion version = _mutations.Read();
            Interlocked.Decrement(ref _state);
            return version;
        }

        MutationVersion nextVersion = _mutations.Advance();
        Volatile.Write(ref _state, Free);
        return nextVersion;
    }

    private struct MutationClock
    {
        private long _version;

        internal MutationVersion Read() => new MutationVersion(Volatile.Read(ref _version));

        internal MutationVersion Advance() =>
            new MutationVersion(Interlocked.Increment(ref _version));
    }
}

internal sealed unsafe class OperationLease<T> : ILifetimeEdge where T : unmanaged
{
    private RustHandle<T>? _owner;
    private ILifetimeEdge[] _dependencies;

    internal OperationLease(RustHandle<T> owner, ILifetimeEdge[] dependencies)
    {
        _owner = owner;
        _dependencies = dependencies;
    }

    public void Release()
    {
        RustHandle<T>? owner = Interlocked.Exchange(ref _owner, null);
        if (owner is null)
        {
            return;
        }

        ILifetimeEdge[] dependencies = Interlocked.Exchange(
            ref _dependencies,
            Array.Empty<ILifetimeEdge>());
        try
        {
            LifetimeEdges.ReleaseLeases(dependencies);
        }
        finally
        {
            owner.ExitOperation();
        }
    }
}

internal sealed unsafe class RustHandle<T> where T : unmanaged
{
    private IntPtr _ptr;
    private readonly RustDestructor<T>? _destructor;
    private readonly WrapperKind _wrapperKind;
    private LifetimeEdges _edges;
    private BorrowLedger _borrows;
    private int _activeOperations;
    private int _closed;

    private RustHandle(
        T* ptr,
        RustDestructor<T>? destructor,
        WrapperKind kind,
        ILifetimeEdge?[] edges)
    {
        _ptr = (IntPtr)ptr;
        _destructor = destructor;
        _wrapperKind = kind;
        _edges = new LifetimeEdges(kind, edges);
    }

    internal static RustHandle<T> Owned(T* ptr, RustDestructor<T> destructor) =>
        Create(ptr, destructor, WrapperKind.Owned, Array.Empty<ILifetimeEdge?>());

    internal static RustHandle<T> Owned(
        T* ptr,
        RustDestructor<T> destructor,
        params ILifetimeEdge?[] edges) =>
        Create(ptr, destructor, WrapperKind.Owned, edges);

    internal static RustHandle<T> Borrowed(
        T* ptr,
        WrapperKind kind,
        params ILifetimeEdge?[] edges) =>
        Create(ptr, null, kind, edges);

    private static RustHandle<T> Create(
        T* ptr,
        RustDestructor<T>? destructor,
        WrapperKind kind,
        ILifetimeEdge?[] edges)
    {
        try
        {
            return new RustHandle<T>(ptr, destructor, kind, edges);
        }
        catch
        {
            try
            {
                if (ptr != null && destructor is not null)
                {
                    destructor(ptr);
                }
            }
            finally
            {
                LifetimeEdges.ReleaseNoThrow(edges);
            }

            throw;
        }
    }

    ~RustHandle()
    {
        try
        {
            ReleaseWrapper();
        }
        catch
        {
        }
    }

    internal T* Ptr => (T*)Volatile.Read(ref _ptr);

    internal bool IsNull => Volatile.Read(ref _closed) != 0 || Volatile.Read(ref _ptr) == IntPtr.Zero;

    internal bool IsCurrent(MutationVersion mutationVersion) =>
        !IsNull && _borrows.IsCurrent(mutationVersion);

    internal BorrowLease<T> Lease(BorrowKind kind)
    {
        if (IsNull)
        {
            throw new ObjectDisposedException(typeof(T).Name);
        }

        if (kind == BorrowKind.Exclusive && _wrapperKind == WrapperKind.SharedView)
        {
            throw new InvalidOperationException("This wrapper only carries a shared borrow.");
        }

        _borrows.Enter(kind);
        try
        {
            OperationLease<T> operation = AcquireOperation();
            return new BorrowLease<T>(this, kind, Ptr, operation);
        }
        catch
        {
            _borrows.Exit(kind);
            throw;
        }
    }

    internal BorrowLease<T> LeaseCurrentVersion(MutationVersion mutationVersion)
    {
        if (!IsCurrent(mutationVersion))
        {
            throw new InvalidOperationException(
                "This borrowed view was invalidated by disposal or mutation of its source.");
        }

        BorrowLease<T> lease = Lease(BorrowKind.Shared);
        if (_borrows.IsCurrent(mutationVersion) && !IsNull)
        {
            return lease;
        }

        lease.Release();
        throw new InvalidOperationException(
            "This borrowed view was invalidated by disposal or mutation of its source.");
    }

    internal OperationLease<T> AcquireOperation()
    {
        if (IsNull)
        {
            throw new ObjectDisposedException(typeof(T).Name);
        }

        Interlocked.Increment(ref _activeOperations);
        if (IsNull)
        {
            ExitOperation();
            throw new ObjectDisposedException(typeof(T).Name);
        }

        try
        {
            return new OperationLease<T>(this, _edges.AcquireOperationLeases());
        }
        catch
        {
            ExitOperation();
            throw;
        }
    }

    internal void ExitBorrow(BorrowKind kind) => _borrows.Exit(kind);

    internal MutationVersion ExitBorrowKeepingReference(BorrowKind kind) => _borrows.Exit(kind);

    internal void ExitOperation() => Interlocked.Decrement(ref _activeOperations);

    internal void ReleaseWrapper()
    {
        if (Volatile.Read(ref _activeOperations) != 0)
        {
            throw new InvalidOperationException(
                "Cannot dispose a native value while an operation is active.");
        }

        if (Interlocked.Exchange(ref _closed, 1) != 0)
        {
            return;
        }

        if (_wrapperKind == WrapperKind.ExclusiveView)
        {
            _borrows.EndScope();
        }

        IntPtr ptr = Interlocked.Exchange(ref _ptr, IntPtr.Zero);
        try
        {
            if (ptr != IntPtr.Zero && _destructor is not null)
            {
                _destructor((T*)ptr);
            }
        }
        finally
        {
            _edges.Release();
            GC.SuppressFinalize(this);
        }
    }
}