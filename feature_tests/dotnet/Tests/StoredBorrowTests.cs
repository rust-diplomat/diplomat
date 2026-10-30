using System;
using System.Runtime.CompilerServices;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class StoredBorrowTests
{
    [MethodImpl(MethodImplOptions.NoInlining)]
    private static void StoreFreshTarget(StoredBorrowHolder holder)
    {
        holder.Store(StoredBorrowTarget.Create());
    }

    [Fact]
    public void ArgumentStoredIntoSelf_StaysAliveWhileHolderIsReachable()
    {
        TestGc.DrainFinalizers();
        StoredBorrowTarget.ResetDropCount();

        StoredBorrowTarget parent = StoredBorrowTarget.Create();
        StoredBorrowHolder holder = parent.MakeHolder();
        StoreFreshTarget(holder);

        TestGc.DrainFinalizers();

        Assert.Equal(0ul, StoredBorrowTarget.DropCount());
        GC.KeepAlive(holder);
        GC.KeepAlive(parent);
    }

    [Fact]
    public void StoringIntoEachOther_IsRejectedBeforeRustKeepsThePointer()
    {
        StoredBorrowNode a = StoredBorrowNode.Create();
        StoredBorrowNode b = StoredBorrowNode.Create();
        a.Link(b);

        Assert.Throws<InvalidOperationException>(() => b.Link(a));
        Assert.Throws<InvalidOperationException>(() => a.Link(a));
    }

    [Fact]
    public void StoringIntoBorrowedView_IsRejected()
    {
        StoredBorrowNode owner = StoredBorrowNode.Create();
        StoredBorrowNode view = owner.View();

        Assert.Throws<InvalidOperationException>(() => view.Link(StoredBorrowNode.Create()));
        GC.KeepAlive(owner);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static StoredBorrowNode AdoptFromTemporaries()
    {
        StoredBorrowNode first = StoredBorrowNode.Create();
        StoredBorrowNode second = StoredBorrowNode.Create();
        first.Link(second);

        StoredBorrowNode adopter = StoredBorrowNode.Create();
        adopter.AdoptLink(first);
        return adopter;
    }

    [Fact]
    public void DataCopiedOutOfArgument_StaysAliveWhileHolderIsReachable()
    {
        TestGc.DrainFinalizers();
        StoredBorrowNode.ResetDropCount();

        StoredBorrowNode adopter = AdoptFromTemporaries();
        TestGc.DrainFinalizers();

        Assert.Equal(0ul, StoredBorrowNode.DropCount());
        GC.KeepAlive(adopter);
    }

    [Fact]
    public void ExclusiveArgumentStoredIntoHolder_StaysBorrowedUntilHolderIsDisposed()
    {
        StoredBorrowTarget target = StoredBorrowTarget.Create();
        StoredBorrowExclusiveHolder holder = StoredBorrowExclusiveHolder.Create();
        StoredBorrowExclusiveHolder other = StoredBorrowExclusiveHolder.Create();

        holder.Store(target);
        Assert.Throws<InvalidOperationException>(() => other.Store(target));

        holder.Dispose();
        other.Store(target);
        other.Dispose();
    }
}
