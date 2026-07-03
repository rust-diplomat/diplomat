using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class StringifierTests
{
    [Fact]
    public void Stringifier_OverridesObjectToString()
    {
        using MyOpaqueEnum value = MyOpaqueEnum.New();
        object boxed = value;

        Assert.Equal("MyOpaqueEnum::A", boxed.ToString());
    }

    [Fact]
    public void Stringifier_RendersFloat64VecContents()
    {
        byte[] bytes =
        [
            0x3f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xc0, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        using Float64Vec vec = Float64Vec.NewF64BeBytes(bytes);

        Assert.Equal("[1.5, -2.25]", vec.ToString());
        Assert.Equal("[1.5, -2.25]", $"{vec}");
    }
}
