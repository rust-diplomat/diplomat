using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Enum discriminant mapping, including negative values — verifies the
// explicit `: int` underlying type the backend now emits.
public class EnumTests
{
    [Fact]
    public void MyEnum_HasExpectedDiscriminants()
    {
        Assert.Equal(-2, (int)MyEnum.A);
        Assert.Equal(-1, (int)MyEnum.B);
        Assert.Equal(0, (int)MyEnum.C);
        Assert.Equal(1, (int)MyEnum.D);
        Assert.Equal(2, (int)MyEnum.E);
    }

    [Fact]
    public void DefaultEnum_HasExpectedDiscriminants()
    {
        Assert.Equal(0, (int)DefaultEnum.A);
        Assert.Equal(1, (int)DefaultEnum.B);
    }

    [Fact]
    public void ContiguousEnum_HasExpectedDiscriminants()
    {
        Assert.Equal(0, (int)ContiguousEnum.C);
        Assert.Equal(1, (int)ContiguousEnum.D);
        Assert.Equal(2, (int)ContiguousEnum.E);
        Assert.Equal(3, (int)ContiguousEnum.F);
    }
}
