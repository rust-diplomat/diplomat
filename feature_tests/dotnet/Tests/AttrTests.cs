using System;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class AttrTests
{
    [Fact]
    public void AttrOpaque_RenamedConstructorMethodsAndAbiName_Work()
    {
        using AttrOpaque1Renamed value = AttrOpaque1Renamed.totally_not_New();

        Assert.Equal((byte)77, value.method_renamed());
        Assert.Equal((byte)123, value.Abirenamed());
        Assert.Equal(10, AttrOpaque1Renamed.MacTest());
        Assert.Equal(0, AttrOpaque1Renamed.Hello());
    }

    [Fact]
    public void AttrOpaque_NamespacedAndUnnamespacedArguments_Work()
    {
        using AttrOpaque1Renamed value = AttrOpaque1Renamed.NewOverload(5);
        using Unnamespaced unnamespaced = Unnamespaced.Make(RenamedAttrEnum.Renamed);

        value.UseNamespaced(RenamedAttrEnum.A);
        value.UseUnnamespaced(unnamespaced);
        unnamespaced.UseNamespaced(value);
    }

    [Fact]
    public void RenamedStructWithAttrs_FallibleConstructorMapsResult()
    {
        RenamedStructWithAttrs value = RenamedStructWithAttrs.NewFallible(true, 32);

        Assert.True(value.A);
        Assert.Equal(32u, value.B);
        Assert.Equal(5u, value.C());
        Assert.Throws<InvalidOperationException>(() => RenamedStructWithAttrs.NewFallible(false, 2));
    }

    [Fact]
    public void RenamedMixinTest_StaticWriterMethodReturnsString()
    {
        Assert.Equal("Hello!", RenamedMixinTest.Hello());
    }
}
