using System;
using System.Threading;

namespace Somelib.Diplomat;

#nullable enable

internal interface IBorrowLease : ILifetimeEdge
{
    BorrowKind Kind { get; }

    ILifetimeEdge AcquireOperation();

    ILifetimeEdge IntoVersionedEdge();
}

internal interface IVersionedReference : ILifetimeEdge
{
    IBorrowLease Lease();
}

internal sealed unsafe class BorrowLease<T> : IBorrowLease where T : unmanaged
{
    private RustHandle<T>? _owner;
    private readonly BorrowKind _kind;
    private OperationLease<T>? _operation;

    internal BorrowLease(
        RustHandle<T> owner,
        BorrowKind kind,
        T* ptr,
        OperationLease<T> operation)
    {
        _owner = owner;
        _kind = kind;
        Ptr = ptr;
        _operation = operation;
    }

    internal T* Ptr { get; }

    public BorrowKind Kind => _kind;

    public ILifetimeEdge AcquireOperation()
    {
        RustHandle<T>? owner = Volatile.Read(ref _owner);
        if (owner is null)
        {
            throw new ObjectDisposedException(nameof(BorrowLease<T>));
        }

        return owner.AcquireOperation();
    }

    public ILifetimeEdge IntoVersionedEdge()
    {
        RustHandle<T> owner = TakeOwner();
        MutationVersion version = owner.ExitBorrowKeepingReference(_kind);
        TakeOperation()?.Release();
        return new VersionedReference(owner, version);
    }

    internal void TransferToPersistent()
    {
        TakeOperation()?.Release();
    }

    public void Release()
    {
        RustHandle<T>? owner = Interlocked.Exchange(ref _owner, null);
        if (owner is null)
        {
            return;
        }

        try
        {
            TakeOperation()?.Release();
        }
        finally
        {
            owner.ExitBorrow(_kind);
        }
    }

    private RustHandle<T> TakeOwner()
    {
        RustHandle<T>? owner = Interlocked.Exchange(ref _owner, null);
        if (owner is null)
        {
            throw new ObjectDisposedException(nameof(BorrowLease<T>));
        }

        return owner;
    }

    private OperationLease<T>? TakeOperation() => Interlocked.Exchange(ref _operation, null);

    private sealed class VersionedReference : IVersionedReference
    {
        private RustHandle<T>? _owner;
        private readonly MutationVersion _version;

        internal VersionedReference(RustHandle<T> owner, MutationVersion version)
        {
            _owner = owner;
            _version = version;
        }

        public IBorrowLease Lease()
        {
            RustHandle<T>? owner = Volatile.Read(ref _owner);
            if (owner is null)
            {
                throw new ObjectDisposedException(nameof(VersionedReference));
            }

            return owner.LeaseCurrentVersion(_version);
        }

        public void Release()
        {
            Interlocked.Exchange(ref _owner, null);
        }
    }
}