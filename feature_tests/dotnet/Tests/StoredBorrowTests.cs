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
    public void StoringTheSameTargetAgain_Works()
    {
        StoredBorrowTarget parent = StoredBorrowTarget.Create();
        StoredBorrowHolder holder = parent.MakeHolder();
        StoredBorrowTarget target = StoredBorrowTarget.Create();

        holder.Store(target);
        holder.Store(target);
    }

    [Fact]
    public void StoringIntoBorrowedView_IsRejected()
    {
        StoredBorrowTarget parent = StoredBorrowTarget.Create();
        StoredBorrowHolder holder = parent.MakeHolder();

        Assert.Throws<InvalidOperationException>(() => holder.View().Store(StoredBorrowTarget.Create()));
        GC.KeepAlive(holder);
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

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static StoredBorrowApple AppleWithDataFromTemporaries()
    {
        StoredBorrowApple apple = StoredBorrowApple.Create();
        apple.Take(StoredBorrowTarget.Create().MakeBanana());
        return apple;
    }

    [Fact]
    public void DataCopiedOutOfArgument_StaysAliveWhileHolderIsReachable()
    {
        TestGc.DrainFinalizers();
        StoredBorrowTarget.ResetDropCount();

        StoredBorrowApple apple = AppleWithDataFromTemporaries();
        TestGc.DrainFinalizers();

        Assert.Equal(0ul, StoredBorrowTarget.DropCount());
        GC.KeepAlive(apple);
    }

    [Fact]
    public void CopiedData_IsInvalidatedWhenWhatItBorrowsIsMutated()
    {
        StoredBorrowTarget target = StoredBorrowTarget.Create();
        StoredBorrowApple apple = StoredBorrowApple.Create();
        apple.Take(target.MakeBanana());

        target.Touch();

        Assert.Throws<InvalidOperationException>(
            () => apple.Take(StoredBorrowTarget.Create().MakeBanana()));
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static StoredBorrowApple AppleKeepingTemporaryBanana()
    {
        StoredBorrowApple apple = StoredBorrowApple.Create();
        apple.Keep(StoredBorrowTarget.Create().MakeBanana());
        return apple;
    }

    [Fact]
    public void StoredArgument_KeepsWhatItBorrowsAlive()
    {
        TestGc.DrainFinalizers();
        StoredBorrowTarget.ResetDropCount();

        StoredBorrowApple apple = AppleKeepingTemporaryBanana();
        TestGc.DrainFinalizers();

        Assert.Equal(0ul, StoredBorrowTarget.DropCount());
        GC.KeepAlive(apple);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static StoredBorrowApple AppleStoringThroughMutableSelf()
    {
        StoredBorrowApple apple = StoredBorrowApple.Create();
        apple.StoreMut(StoredBorrowTarget.Create());
        return apple;
    }

    [Fact]
    public void ArgumentStoredThroughMutableSelf_StaysAliveWhileHolderIsReachable()
    {
        TestGc.DrainFinalizers();
        StoredBorrowTarget.ResetDropCount();

        StoredBorrowApple apple = AppleStoringThroughMutableSelf();
        TestGc.DrainFinalizers();

        Assert.Equal(0ul, StoredBorrowTarget.DropCount());
        GC.KeepAlive(apple);
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static void LendFreshTarget(StoredBorrowHolder holder)
    {
        StoredBorrowTarget.Create().LendTo(holder);
    }

    [Fact]
    public void ArgumentStoredAndReturned_IsKeptByTheHolderAlone()
    {
        TestGc.DrainFinalizers();
        StoredBorrowTarget.ResetDropCount();

        StoredBorrowTarget parent = StoredBorrowTarget.Create();
        StoredBorrowHolder holder = parent.MakeHolder();
        LendFreshTarget(holder);
        TestGc.DrainFinalizers();

        Assert.Equal(0ul, StoredBorrowTarget.DropCount());
        GC.KeepAlive(holder);
        GC.KeepAlive(parent);
    }

    [Fact]
    public void ArgumentStoredAndReturned_InvalidatesBothWhenMutated()
    {
        StoredBorrowTarget parent = StoredBorrowTarget.Create();
        StoredBorrowHolder holder = parent.MakeHolder();
        StoredBorrowTarget target = StoredBorrowTarget.Create();
        StoredBorrowBanana banana = target.LendTo(holder);

        target.Touch();

        Assert.Throws<InvalidOperationException>(() => holder.Store(StoredBorrowTarget.Create()));
        Assert.Throws<InvalidOperationException>(() => StoredBorrowApple.Create().Take(banana));
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static void StoreFreshTargetExclusively(StoredBorrowExclusiveHolder holder)
    {
        holder.Store(StoredBorrowTarget.Create());
    }

    [Fact]
    public void ExclusiveArgumentStoredIntoHolder_StaysAliveWhileHolderIsReachable()
    {
        TestGc.DrainFinalizers();
        StoredBorrowTarget.ResetDropCount();

        StoredBorrowExclusiveHolder holder = StoredBorrowExclusiveHolder.Create();
        StoreFreshTargetExclusively(holder);
        TestGc.DrainFinalizers();

        Assert.Equal(0ul, StoredBorrowTarget.DropCount());
        holder.Dispose();
    }

    [Fact]
    public void ExclusiveArgumentStoredWithAValueReturned_StaysBorrowedUntilHolderIsDisposed()
    {
        StoredBorrowTarget target = StoredBorrowTarget.Create();
        StoredBorrowExclusiveHolder holder = StoredBorrowExclusiveHolder.Create();
        StoredBorrowExclusiveHolder other = StoredBorrowExclusiveHolder.Create();

        Assert.Equal(1u, holder.StoreAndCount(target));
        Assert.Throws<InvalidOperationException>(() => other.Store(target));

        holder.Dispose();
        other.Store(target);
        other.Dispose();
    }

    [MethodImpl(MethodImplOptions.NoInlining)]
    private static void StoreFreshTargetInsideExclusiveApple(
        StoredBorrowExclusiveHolder holder,
        StoredBorrowApple apple)
    {
        holder.StoreChain(apple, StoredBorrowTarget.Create());
    }

    [Fact]
    public void ExclusiveArgumentThatStoresAnother_KeepsItAfterItsHolderIsDisposed()
    {
        TestGc.DrainFinalizers();
        StoredBorrowTarget.ResetDropCount();

        StoredBorrowApple apple = StoredBorrowApple.Create();
        StoredBorrowExclusiveHolder holder = StoredBorrowExclusiveHolder.Create();
        StoreFreshTargetInsideExclusiveApple(holder, apple);
        holder.Dispose();
        TestGc.DrainFinalizers();

        Assert.Equal(0ul, StoredBorrowTarget.DropCount());
        GC.KeepAlive(apple);
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
}
