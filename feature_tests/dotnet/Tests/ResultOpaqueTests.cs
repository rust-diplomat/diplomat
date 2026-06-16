using System;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Result<T, E> lowering across every error shape the backend supports:
// enum error, struct error, and unit error — each mapped to the right
// thrown exception, plus the Ok paths (opaque and primitive).
public class ResultOpaqueTests
{
    [Fact]
    public void New_Ok_RoundTripsInteger()
    {
        using ResultOpaque r = ResultOpaque.New(5);
        // Rust `assert_eq!(i, self.0)` panics on mismatch.
        r.AssertInteger(5);
    }

    [Fact]
    public void NewInt_Ok_ReturnsValue()
    {
        Assert.Equal(7, ResultOpaque.NewInt(7));
    }

    [Fact]
    public void NewFailingFoo_ThrowsEnumExceptionWithFooInner()
    {
        ErrorEnumException ex =
            Assert.Throws<ErrorEnumException>(() => ResultOpaque.NewFailingFoo());
        Assert.Equal(ErrorEnum.Foo, ex.Inner);
    }

    [Fact]
    public void NewFailingBar_ThrowsEnumExceptionWithBarInner()
    {
        ErrorEnumException ex =
            Assert.Throws<ErrorEnumException>(() => ResultOpaque.NewFailingBar());
        Assert.Equal(ErrorEnum.Bar, ex.Inner);
    }

    [Fact]
    public void NewFailingStruct_ThrowsStructExceptionCarryingFields()
    {
        ErrorStructException ex =
            Assert.Throws<ErrorStructException>(() => ResultOpaque.NewFailingStruct(3));
        Assert.Equal(3, ex.Inner.I);
        Assert.Equal(12, ex.Inner.J);
    }

    [Fact]
    public void NewFailingUnit_ThrowsInvalidOperation()
    {
        // Unit error `Result<_, ()>` maps to the built-in BCL exception.
        Assert.Throws<InvalidOperationException>(() => ResultOpaque.NewFailingUnit());
    }
}
