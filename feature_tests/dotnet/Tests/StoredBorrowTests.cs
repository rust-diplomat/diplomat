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
    public void LinkingTheSamePairAgain_IsNotACycle()
    {
        StoredBorrowNode a = StoredBorrowNode.Create();
        StoredBorrowNode b = StoredBorrowNode.Create();

        a.Link(b);
        a.Link(b);
        a.AdoptLink(b);
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
    public void StoredReference_IsInvalidatedWhenItsSourceIsMutated()
    {
        StoredBorrowTarget parent = StoredBorrowTarget.Create();
        StoredBorrowHolder holder = parent.MakeHolder();
        StoredBorrowTarget target = StoredBorrowTarget.Create();
        holder.Store(target);

        target.Touch();

        Assert.Throws<InvalidOperationException>(() => holder.Store(StoredBorrowTarget.Create()));
    }

    [Fact]
    public void CopiedData_IsInvalidatedWhenWhatItBorrowsIsMutated()
    {
        StoredBorrowTarget parent = StoredBorrowTarget.Create();
        StoredBorrowHolder source = parent.MakeHolder();
        StoredBorrowHolder adopter = StoredBorrowTarget.Create().MakeHolder();
        adopter.Adopt(source);

        parent.Touch();

        Assert.Throws<InvalidOperationException>(
            () => adopter.Adopt(StoredBorrowTarget.Create().MakeHolder()));
    }

    [Fact]
    public void CopiedData_SurvivesMutationOfTheArgumentItCameFrom()
    {
        StoredBorrowTarget parent = StoredBorrowTarget.Create();
        StoredBorrowHolder source = parent.MakeHolder();
        StoredBorrowHolder adopter = StoredBorrowTarget.Create().MakeHolder();
        adopter.Adopt(source);

        source.Touch();

        adopter.Adopt(StoredBorrowTarget.Create().MakeHolder());
        GC.KeepAlive(parent);
    }

    [Fact]
    public void PassingAViewWhereDataMayBeCopied_IsRejected()
    {
        StoredBorrowNode owner = StoredBorrowNode.Create();
        StoredBorrowNode other = StoredBorrowNode.Create();

        Assert.Throws<InvalidOperationException>(() => owner.View().AdoptLink(StoredBorrowNode.Create()));
        Assert.Throws<InvalidOperationException>(() => other.Link(owner.View()));
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

    [Fact]
    public void ExclusiveArgumentStoredBeforeAnError_StaysBorrowedUntilHolderIsDisposed()
    {
        StoredBorrowTarget target = StoredBorrowTarget.Create();
        StoredBorrowExclusiveHolder holder = StoredBorrowExclusiveHolder.Create();
        StoredBorrowExclusiveHolder other = StoredBorrowExclusiveHolder.Create();

        Assert.ThrowsAny<Exception>(() => holder.StoreThenFail(target));
        Assert.Throws<InvalidOperationException>(() => other.Store(target));

        holder.Dispose();
        other.Store(target);
        other.Dispose();
    }

    [Fact]
    public void ValueThatMayHoldItself_CannotBeMutated()
    {
        StoredBorrowNode a = StoredBorrowNode.Create();
        StoredBorrowNode b = StoredBorrowNode.Create();
        a.Link(b);

        Assert.Throws<InvalidOperationException>(() => b.Touch());
        a.Touch();
    }

    [Fact]
    public void DataCopiedFromAValueThatMayHoldItself_KeepsThatValueLocked()
    {
        StoredBorrowNode a = StoredBorrowNode.Create();
        StoredBorrowNode b = StoredBorrowNode.Create();
        StoredBorrowNode c = StoredBorrowNode.Create();
        a.Link(b);
        c.AdoptLink(b);

        Assert.Throws<InvalidOperationException>(() => b.Touch());
        c.View();
    }
}
