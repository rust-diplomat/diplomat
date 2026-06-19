using Somelib;
using Xunit;

namespace Somelib.FeatureTests;

public class RenamedVectorTestTests
{
    [Fact]
    public void PushLenAndGet_Work()
    {
        using RenamedVectorTest vector = RenamedVectorTest.New();

        Assert.Equal((nuint)0, vector.Len());
        Assert.Null(vector.Get(0));

        vector.Push(1.5);
        vector.Push(2.25);

        Assert.Equal((nuint)2, vector.Len());
        Assert.Equal(1.5, vector.Get(0));
        Assert.Equal(2.25, vector.Get(1));
        Assert.Null(vector.Get(2));
    }
}
