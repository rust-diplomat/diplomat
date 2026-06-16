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
}
