using System;
using System.Text;
using Somelib;
using Somelib.Diplomat;
using Xunit;

namespace Somelib.FeatureTests;

[Collection(RcSharedNativeStateCollection.Name)]
public class RuntimeBorrowSafetyTests
{
    private static byte[] Utf8(string value) => Encoding.UTF8.GetBytes(value);

    [Fact]
    public void OwnedChildOfSharedBorrow_DoesNotBlockSource_AndIsInvalidatedByMutation()
    {
        OpaqueThinVec source = OpaqueThinVec.CreateSingle(7, 1.5f, Utf8("hi"));
        OpaqueThinIter iter = source.Iter();

        // The iterator is a read view: mutating the source succeeds and stales it.
        source.FirstC = "changed";

        Assert.Equal((nuint)1, source.Len());
        Assert.Throws<InvalidOperationException>(() => iter.Next());
        GC.KeepAlive(iter);
    }

    [Fact]
    public void OwnedChildOfSharedBorrow_ReadsUntilSourceIsMutated()
    {
        OpaqueThinVec source = OpaqueThinVec.CreateSingle(7, 1.5f, Utf8("hi"));
        OpaqueThinIter iter = source.Iter();

        using OpaqueThin first = iter.Next()!;
        Assert.Equal(7, first.A);
        Assert.Null(iter.Next());
        GC.KeepAlive(source);
    }

    [Fact]
    public void ExclusiveView_BlocksSourceUntilDisposed()
    {
        ExclusiveSource source = ExclusiveSource.Create(1);

        using (ExclusiveView view = source.ViewMut())
        {
            Assert.Throws<InvalidOperationException>(() => source.Value());
            Assert.Throws<InvalidOperationException>(() => source.SetValue(9));
            view.Add(2);
            Assert.Equal(3ul, view.Value());
        }

        Assert.Equal(3ul, source.Value());
    }

    [Fact]
    public void ExclusiveOwnedChild_BlocksSourceUntilDisposed()
    {
        ExclusiveSource source = ExclusiveSource.Create(1);

        using (ExclusiveWriter writer = source.TakeWriter())
        {
            Assert.Throws<InvalidOperationException>(() => source.SetValue(9));
            writer.Add(4);
        }

        Assert.Equal(5ul, source.Value());
        source.SetValue(9);
        Assert.Equal(9ul, source.Value());
    }

    [Fact]
    public void ExclusiveReturns_AreDisposable_AndBorrowingSourcesAreFinalizerOnly()
    {
        Assert.Contains(typeof(IDisposable), typeof(ExclusiveView).GetInterfaces());
        Assert.Contains(typeof(IDisposable), typeof(ExclusiveWriter).GetInterfaces());
        Assert.DoesNotContain(typeof(IDisposable), typeof(ExclusiveSource).GetInterfaces());
        Assert.DoesNotContain(typeof(IDisposable), typeof(OpaqueThinVec).GetInterfaces());
        Assert.DoesNotContain(typeof(IDisposable), typeof(OpaqueThinIter).GetInterfaces());
        Assert.DoesNotContain(typeof(IDisposable), typeof(MyString).GetInterfaces());
    }

    [Fact]
    public void SpanAccess_ReentrantMutationAndCallbackFailureAreRejected()
    {
        MyString value = MyString.New(Utf8("before"));
        using DiplomatBorrowedSpan<byte> view = value.Borrow();

        Assert.Throws<InvalidOperationException>(() =>
            view.WithSpan(_ => value.Str = "nested"));

        Assert.Throws<InvalidOperationException>(() =>
            view.WithSpan(_ => throw new InvalidOperationException("callback failure")));

        value.Str = "after";
        Assert.Throws<InvalidOperationException>(() => view.Clone());
    }

    [Fact]
    public void SpanDispose_IsIdempotentAndSourceMutationInvalidatesAccess()
    {
        MyString value = MyString.New(Utf8("value"));
        DiplomatBorrowedSpan<byte> view = value.Borrow();

        value.Str = "changed";
        Assert.Throws<InvalidOperationException>(() => view.WithSpan(_ => { }));

        view.Dispose();
        view.Dispose();
    }

    [Fact]
    public void SpanDispose_DuringWithSpanThrowsWithoutChangingState()
    {
        MyString value = MyString.New(Utf8("value"));
        DiplomatBorrowedSpan<byte> view = value.Borrow();

        view.WithSpan(span =>
        {
            Assert.Throws<InvalidOperationException>(() => view.Dispose());
            Assert.Equal((byte)'v', span[0]);
        });

        Assert.Equal(Utf8("value"), view.Clone());
        view.Dispose();
        Assert.Throws<ObjectDisposedException>(() => view.Clone());
    }
}
