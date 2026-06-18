using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// Value struct round-trip: field marshalling (byte/bool/ulong/int/uint/enum),
// a by-value method (`IntoA`), and method overloading.
public class MyStructTests
{
    [Fact]
    public void New_HasCanonicalFieldValues()
    {
        MyStruct s = MyStruct.New();
        Assert.Equal((byte)17, s.A);
        Assert.True(s.B);
        Assert.Equal((byte)209, s.C);
        Assert.Equal(1234ul, s.D);
        Assert.Equal(5991, s.E);
        Assert.Equal((uint)'餐', s.F); // Rust `'餐' as DiplomatChar` (U+9910)
        Assert.Equal(MyEnum.B, s.G);
    }

    [Fact]
    public void IntoA_ReturnsFieldA()
    {
        Assert.Equal((byte)17, MyStruct.New().IntoA());
    }

    [Fact]
    public void NewOverload_OverridesE()
    {
        Assert.Equal(42, MyStruct.NewOverload(42).E);
    }

    [Fact]
    public void ReturnsZstResult_MapsOkUnit()
    {
        MyStruct.ReturnsZstResult();
    }

    [Fact]
    public void FailsZstResult_ThrowsZstException()
    {
        Assert.Throws<MyZstException>(() => MyStruct.FailsZstResult());
    }
}
