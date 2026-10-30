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
}
