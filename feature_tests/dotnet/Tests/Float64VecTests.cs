using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class Float64VecTests
{
    [Fact]
    public void NewF64BeBytes_PinsByteSliceAndExposesValues()
    {
        byte[] bytes =
        [
            0x3f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xc0, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        using Float64Vec vec = Float64Vec.NewF64BeBytes(bytes);

        Assert.Equal(1.5, vec.Get((nuint)0));
        Assert.Equal(-2.25, vec.Get((nuint)1));
        Assert.Null(vec.Get((nuint)2));
        Assert.Equal("[1.5, -2.25]", vec.ToString());
    }
}
