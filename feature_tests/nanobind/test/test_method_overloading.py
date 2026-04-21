import somelib

def test_named_constructor_overloading():
    """Test that named constructors with method overloading work correctly.

    This tests the fix for the panic that occurred when multiple named constructors
    were renamed to Python keywords (like "from") which get escaped to "from_".

    The MethodOverloading class has three overloaded from_() methods that take int32, int64, and uint32.
    """
    # Test int32 overload
    d1 = somelib.mylib.MethodOverloading.from_(42)
    assert d1 is not None, "from_(int32) should create a MethodOverloading"

    # Test int64 overload
    d2 = somelib.mylib.MethodOverloading.from_(9999999999)
    assert d2 is not None, "from_(int64) should create a MethodOverloading"

    # Test uint32 overload (positive int should work)
    d3 = somelib.mylib.MethodOverloading.from_(12345)
    assert d3 is not None, "from_(uint32) should create a MethodOverloading"

    # All three overloads should work and create distinct objects
    assert d1 is not d2, "Different calls should create different objects"
    assert d2 is not d3, "Different calls should create different objects"
    assert d1 is not d3, "Different calls should create different objects"
