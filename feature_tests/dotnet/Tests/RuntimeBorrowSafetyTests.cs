using System;
using System.Threading;
using System.Threading.Tasks;
using Somelib;
using Somelib.Diplomat;
using Xunit;

namespace Somelib.FeatureTests;

[Collection(RcSharedNativeStateCollection.Name)]
public sealed class RuntimeBorrowSafetyTests
{
    private static async Task AwaitWithTimeout(Task task)
    {
        Task completed = await Task.WhenAny(task, Task.Delay(TimeSpan.FromSeconds(5)));
        Assert.Same(task, completed);
        await task;
    }

    [Fact]
    public async Task ConcurrentSharedCalls_AreAllowedButMutableCallsFailFast()
    {
        BorrowSafetyProbe.ResetSharedCall();
        using BorrowSafetyProbe probe = BorrowSafetyProbe.Create();
        Task heldCall = Task.Factory.StartNew(
            probe.HoldShared,
            CancellationToken.None,
            TaskCreationOptions.LongRunning,
            TaskScheduler.Default
        );

        try
        {
            Assert.True(
                SpinWait.SpinUntil(BorrowSafetyProbe.SharedCallEntered, TimeSpan.FromSeconds(5))
            );
            Assert.True(probe.PingShared());
            Assert.Throws<InvalidOperationException>(() => probe.PingMutable());
        }
        finally
        {
            BorrowSafetyProbe.ReleaseSharedCall();
            await AwaitWithTimeout(heldCall);
        }
    }

    [Fact]
    public async Task ConcurrentMutableCall_RejectsSharedAndMutableCalls()
    {
        BorrowSafetyProbe.ResetMutableCall();
        using BorrowSafetyProbe probe = BorrowSafetyProbe.Create();
        Task heldCall = Task.Factory.StartNew(
            probe.HoldMutable,
            CancellationToken.None,
            TaskCreationOptions.LongRunning,
            TaskScheduler.Default
        );

        try
        {
            Assert.True(
                SpinWait.SpinUntil(BorrowSafetyProbe.MutableCallEntered, TimeSpan.FromSeconds(5))
            );
            Assert.Throws<InvalidOperationException>(() => probe.PingShared());
            Assert.Throws<InvalidOperationException>(() => probe.PingMutable());
        }
        finally
        {
            BorrowSafetyProbe.ReleaseMutableCall();
            await AwaitWithTimeout(heldCall);
        }

        Assert.True(probe.PingMutable());
    }

    [Fact]
    public async Task Dispose_DuringActiveMutableCall_DefersNativeDestruction()
    {
        BorrowSafetyProbe.ResetMutableCall();
        BorrowSafetyProbe.ResetDropCount();
        BorrowSafetyProbe probe = BorrowSafetyProbe.Create();
        Task heldCall = Task.Factory.StartNew(
            probe.HoldMutable,
            CancellationToken.None,
            TaskCreationOptions.LongRunning,
            TaskScheduler.Default
        );

        try
        {
            Assert.True(
                SpinWait.SpinUntil(BorrowSafetyProbe.MutableCallEntered, TimeSpan.FromSeconds(5))
            );
            probe.Dispose();
            Assert.Equal(0ul, BorrowSafetyProbe.DropCount());
        }
        finally
        {
            BorrowSafetyProbe.ReleaseMutableCall();
            await AwaitWithTimeout(heldCall);
        }

        Assert.Equal(1ul, BorrowSafetyProbe.DropCount());
    }

    [Fact]
    public void BorrowedSpan_TransfersOnlyPresentOptionalSourceLeases()
    {
        BorrowSafetyProbe.ResetDropCount();
        BorrowSafetyProbe source = BorrowSafetyProbe.Create();
        DiplomatBorrowedSpan<byte> span = BorrowSafetyProbe.BorrowStaticFromOptional(source, null);

        Assert.Equal(new byte[] { 1, 2, 3 }, span.Clone());

        source.Dispose();
        Assert.Equal(0ul, BorrowSafetyProbe.DropCount());

        span.Dispose();
        Assert.Equal(1ul, BorrowSafetyProbe.DropCount());

        using DiplomatBorrowedSpan<byte> staticSpan =
            BorrowSafetyProbe.BorrowStaticFromOptional(null, null);
        Assert.Equal(new byte[] { 1, 2, 3 }, staticSpan.Clone());
    }

    [Fact]
    public void BorrowedSpan_SameOptionalSourceTwice_ReleasesBothClaimsAndInvalidates()
    {
        BorrowSafetyProbe.ResetDropCount();
        BorrowSafetyProbe source = BorrowSafetyProbe.Create();
        DiplomatBorrowedSpan<byte> span =
            BorrowSafetyProbe.BorrowStaticFromOptional(source, source);

        Assert.Equal(new byte[] { 1, 2, 3 }, span.Clone());
        Assert.True(source.PingMutable());
        Assert.Throws<InvalidOperationException>(() => span.Clone());

        source.Dispose();
        Assert.Equal(0ul, BorrowSafetyProbe.DropCount());

        span.Dispose();
        Assert.Equal(1ul, BorrowSafetyProbe.DropCount());
    }

    [Fact]
    public async Task ExclusiveView_DisposedDuringInFlightCall_ReleasesSourceOnlyAfterCallReturns()
    {
        BorrowSafetyProbe.ResetMutableCall();
        using BorrowSafetyProbe probe = BorrowSafetyProbe.Create();
        BorrowSafetyProbe view = probe.ViewMut();
        Task heldCall = Task.Factory.StartNew(
            view.HoldMutable,
            CancellationToken.None,
            TaskCreationOptions.LongRunning,
            TaskScheduler.Default
        );

        try
        {
            Assert.True(
                SpinWait.SpinUntil(BorrowSafetyProbe.MutableCallEntered, TimeSpan.FromSeconds(5))
            );
            view.Dispose();
            Assert.Throws<InvalidOperationException>(() => probe.PingMutable());
            Assert.Throws<ObjectDisposedException>(() => view.PingMutable());
        }
        finally
        {
            BorrowSafetyProbe.ReleaseMutableCall();
            await AwaitWithTimeout(heldCall);
        }

        Assert.True(probe.PingMutable());
    }

    [Fact]
    public async Task SharedView_CallDuringSourceExclusiveCall_Throws_ThenStaysInvalidated()
    {
        BorrowSafetyProbe.ResetMutableCall();
        using BorrowSafetyProbe probe = BorrowSafetyProbe.Create();
        using BorrowSafetyProbe view = probe.View();
        Assert.True(view.PingShared());

        Task heldCall = Task.Factory.StartNew(
            probe.HoldMutable,
            CancellationToken.None,
            TaskCreationOptions.LongRunning,
            TaskScheduler.Default
        );

        try
        {
            Assert.True(
                SpinWait.SpinUntil(BorrowSafetyProbe.MutableCallEntered, TimeSpan.FromSeconds(5))
            );
            Assert.Throws<InvalidOperationException>(() => view.PingShared());
        }
        finally
        {
            BorrowSafetyProbe.ReleaseMutableCall();
            await AwaitWithTimeout(heldCall);
        }

        Assert.Throws<InvalidOperationException>(() => view.PingShared());
        using BorrowSafetyProbe refreshed = probe.View();
        Assert.True(refreshed.PingShared());
    }
}
