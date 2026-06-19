using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class MethodOverloadingTests
{
    [Fact]
    public void From_OverloadsConstructOpaqueValues()
    {
        using MethodOverloading fromInt = MethodOverloading.from(1);
        using MethodOverloading fromLong = MethodOverloading.from(1L);
        using MethodOverloading fromUInt = MethodOverloading.from(1u);

        Assert.NotNull(fromInt);
        Assert.NotNull(fromLong);
        Assert.NotNull(fromUInt);
    }
}
