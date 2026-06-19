using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class MyOpaqueEnumTests
{
    [Fact]
    public void ToString_UsesOpaqueEnumStringifier()
    {
        using MyOpaqueEnum value = MyOpaqueEnum.New();

        Assert.Equal("MyOpaqueEnum::A", value.ToString());
    }
}
