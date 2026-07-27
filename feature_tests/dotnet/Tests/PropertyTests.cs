using System.Linq;
using System.Reflection;
using System.Text;
using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

// `#[diplomat::attr(auto, getter)]` / `setter` render as C# properties, the same
// HIR mechanism the Dart and JS backends use. These run the accessors, so a
// property that forwards to the wrong method — or that never gets emitted at
// all — fails here and not just in a codegen string assertion.
public class PropertyTests
{
    private static byte[] Utf8(string s) => Encoding.UTF8.GetBytes(s);

    // `get_str` writes a string and `set_str` takes `&DiplomatStr`. In accessor
    // position both are `string`, so they share one property — a getter and
    // setter can never disagree on type.
    [Fact]
    public void GetterAndSetterPair_RoundTripsThroughOneProperty()
    {
        using MyString value = MyString.New(Utf8("before"));

        value.Str = "after 餐";

        Assert.Equal("after 餐", value.Str);
    }

    // `set_first_c` has no getter to pair with, so it can only render as a
    // write-only property — legal C#, and the shape a Rust config object that
    // only has setters depends on.
    [Fact]
    public void SetterWithoutAGetter_WritesThroughAWriteOnlyProperty()
    {
        using OpaqueThinVec vec = OpaqueThinVec.CreateSingle(7, 1.5f, Utf8("before"));

        vec.FirstC = "after";

        using OpaqueThin first = vec.First!;
        Assert.Equal("after", first.C);
    }

    // A non-accessor `&DiplomatStr` parameter keeps its zero-copy `byte[]`
    // shape; only accessors present as `string`. `MyString.New` takes the same
    // Rust type the `Str` setter does, so if the two ever converged this would
    // stop compiling.
    [Fact]
    public void NonAccessorByteParameter_KeepsItsByteArrayShape()
    {
        MethodInfo? create = typeof(MyString).GetMethod("New", BindingFlags.Public | BindingFlags.Static);

        Assert.NotNull(create);
        Assert.Equal(typeof(byte[]), create!.GetParameters().Single().ParameterType);
        Assert.Equal(typeof(string), typeof(MyString).GetProperty("Str")!.PropertyType);
    }

    [Fact]
    public void PrimitivePair_RoundTrips()
    {
        using PropertyMarshals value = PropertyMarshals.Create();

        value.Number = 4242;

        Assert.Equal(4242u, value.Number);
    }

    [Fact]
    public void EnumPair_RoundTrips()
    {
        using PropertyMarshals value = PropertyMarshals.Create();

        Assert.Equal(DefaultEnum.A, value.Choice);
        value.Choice = DefaultEnum.B;

        Assert.Equal(DefaultEnum.B, value.Choice);
    }

    // A struct crosses by value in both directions, bridged through `AsFFI` on
    // the way in and `FromFFI` on the way out.
    [Fact]
    public void StructPair_RoundTrips()
    {
        using PropertyMarshals value = PropertyMarshals.Create();

        value.Point = new PrimitiveStruct { X = 1.5f, A = true, B = 7, C = -9, D = 11, E = 255 };

        PrimitiveStruct read = value.Point;
        Assert.Equal(1.5f, read.X);
        Assert.True(read.A);
        Assert.Equal(7u, read.B);
        Assert.Equal(-9L, read.C);
        Assert.Equal(11, read.D);
        Assert.Equal((byte)255, read.E);
    }

    // The getter returns `Box<Opaque>` and the setter takes `&Opaque`, so
    // ownership differs across the two halves while the C# type does not — which
    // is why they share one property.
    [Fact]
    public void OpaquePair_RoundTripsAcrossDifferingOwnership()
    {
        using PropertyMarshals value = PropertyMarshals.Create();
        using Opaque assigned = Opaque.FromStr("held");

        value.Held = assigned;

        using Opaque read = value.Held;
        Assert.Equal("\"held\"", read.GetDebugStr());
    }

    // The getter hands back a Rust allocation of its own, so a later write
    // through the setter must not reach through it.
    [Fact]
    public void OpaqueGetter_ReturnsAnIndependentValue()
    {
        using PropertyMarshals value = PropertyMarshals.Create();
        using Opaque first = Opaque.FromStr("first");
        value.Held = first;
        using Opaque read = value.Held;

        using Opaque second = Opaque.FromStr("second");
        value.Held = second;

        Assert.Equal("\"first\"", read.GetDebugStr());
    }

    // A written-UTF-8 getter and a validated-UTF-8 (`&str`) setter: Rust may
    // assume the bytes are well formed, so the binding transcodes a real string.
    [Fact]
    public void ValidatedUtf8Pair_RoundTrips()
    {
        using PropertyMarshals value = PropertyMarshals.Create();

        value.Utf8Text = "validated 餐";

        Assert.Equal("validated 餐", value.Utf8Text);
    }

    // The UTF-16 setter pins the C# string in place rather than transcoding it.
    [Fact]
    public void Utf16Pair_RoundTrips()
    {
        using PropertyMarshals value = PropertyMarshals.Create();

        value.Utf16Text = "pinned 餐";

        Assert.Equal("pinned 餐", value.Utf16Text);
    }

    // Both setters write the same Rust `String`, so writing through one marshal
    // and reading through the other proves every text encoding presents as one
    // `string` however it crosses the boundary.
    [Fact]
    public void EveryTextMarshal_PresentsAsTheSameString()
    {
        using PropertyMarshals value = PropertyMarshals.Create();

        value.Utf16Text = "written as UTF-16 餐";
        Assert.Equal("written as UTF-16 餐", value.Utf8Text);

        value.Utf8Text = "written as UTF-8 餐";
        Assert.Equal("written as UTF-8 餐", value.Utf16Text);
    }

    // A struct is a value type with nothing to dispose, so its accessors skip the
    // disposed check an opaque's carry. `c()` takes `self` by value and returns a
    // constant; running it proves the struct template's property path works at
    // all, which nothing else here does.
    [Fact]
    public void StructProperty_ReadsWithoutADisposedCheck()
    {
        RenamedStructWithAttrs value = RenamedStructWithAttrs.NewFallible(true, 17);

        Assert.Equal(5u, value.C);
    }
}
