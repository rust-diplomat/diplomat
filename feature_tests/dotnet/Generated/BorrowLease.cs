using System;
using System.Threading;

namespace Somelib.Diplomat;

#nullable enable

internal interface IBorrowLease : ILifetimeEdge
{
    BorrowKind Kind { get; }

    ILifetimeSource? Source { get; }

    ILifetimeEdge HoldForCall();

    ILifetimeEdge IntoVersionedEdge();
}

internal interface IVersionDependency : ILifetimeEdge
{
    ILifetimeSource? Source { get; }

    ILifetimeEdge Copy();

    ILifetimeEdge CopyAsGuard();
}

internal interface IVersionedReference : IVersionDependency
{
    IBorrowLease LeaseForOperation();
}

// A value may depend on its own version; entering its own ledger or following its own edges for
// that would never end.
internal interface IVersionGuard : IVersionDependency
{
    ILifetimeEdge? Check();
}

internal sealed unsafe class BorrowLease<T> : IBorrowLease where T : unmanaged
{
    private RustHandle<T>? _owner;
    private readonly BorrowKind _kind;
    private ILifetimeEdge? _operation;

    internal BorrowLease(
        RustHandle<T> owner,
        BorrowKind kind,
        T* ptr,
        ILifetimeEdge operation)
    {
        _owner = owner;
        _kind = kind;
        Ptr = ptr;
        _operation = operation;
    }

    internal T* Ptr { get; }

    public BorrowKind Kind => _kind;

    public ILifetimeSource? Source => Volatile.Read(ref _owner);

    internal RustHandle<T> Owner =>
        Volatile.Read(ref _owner) ?? throw new ObjectDisposedException(nameof(BorrowLease<T>));

    internal static ILifetimeEdge VersionedEdge(RustHandle<T> owner, MutationVersion version) =>
        new VersionedReference(owner, version);

    internal static ILifetimeEdge GuardEdge(RustHandle<T> owner, MutationVersion version) =>
        new VersionGuard(owner, version);

    public ILifetimeEdge HoldForCall()
    {
        RustHandle<T>? owner = Volatile.Read(ref _owner);
        if (owner is null)
        {
            throw new InvalidOperationException(
                "The source of this borrowed value is no longer available.");
        }

        return owner.HoldForCall();
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

    private ILifetimeEdge? TakeOperation() => Interlocked.Exchange(ref _operation, null);

    private sealed class VersionedReference : IVersionedReference
    {
        private RustHandle<T>? _owner;
        private readonly MutationVersion _version;

        internal VersionedReference(RustHandle<T> owner, MutationVersion version)
        {
            _owner = owner;
            _version = version;
        }

        public ILifetimeSource? Source => Volatile.Read(ref _owner);

        public ILifetimeEdge Copy()
        {
            RustHandle<T>? owner = Volatile.Read(ref _owner);
            if (owner is null)
            {
                throw new InvalidOperationException(
                    "The source of this borrowed value is no longer available.");
            }

            return new VersionedReference(owner, _version);
        }

        public ILifetimeEdge CopyAsGuard()
        {
            RustHandle<T>? owner = Volatile.Read(ref _owner);
            if (owner is null)
            {
                throw new InvalidOperationException(
                    "The source of this borrowed value is no longer available.");
            }

            return new VersionGuard(owner, _version);
        }

        public IBorrowLease LeaseForOperation()
        {
            RustHandle<T>? owner = Volatile.Read(ref _owner);
            if (owner is null)
            {
                throw new InvalidOperationException(
                    "The source of this borrowed value is no longer available.");
            }

            return owner.LeaseVersionForOperation(_version);
        }

        public void Release()
        {
            Interlocked.Exchange(ref _owner, null);
        }
    }
    private sealed class VersionGuard : IVersionGuard
    {
        private RustHandle<T>? _owner;
        private readonly MutationVersion _version;

        internal VersionGuard(RustHandle<T> owner, MutationVersion version)
        {
            _owner = owner;
            _version = version;
        }

        public ILifetimeSource? Source => Volatile.Read(ref _owner);

        public ILifetimeEdge Copy() => new VersionedReference(Owner(), _version);

        public ILifetimeEdge CopyAsGuard() => new VersionGuard(Owner(), _version);

        private RustHandle<T> Owner() =>
            Volatile.Read(ref _owner)
            ?? throw new InvalidOperationException(
                "The source of this borrowed value is no longer available.");

        public ILifetimeEdge? Check()
        {
            RustHandle<T>? owner = Volatile.Read(ref _owner);
            if (owner is null || !owner.IsVersionAccessible(_version))
            {
                throw new InvalidOperationException(
                    "This value was invalidated by mutation of data it borrows.");
            }

            return null;
        }

        public void Release()
        {
            Interlocked.Exchange(ref _owner, null);
        }
    }
}