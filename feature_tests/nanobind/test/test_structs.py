import somelib
def test_structs():
    o = somelib.Opaque.new_()
    s = somelib.MyStruct.new_()

    o.assert_struct(s)

    assert s.a == 17, "struct values"
    assert s.b == True, "struct values"
    assert s.c == 209, "struct values"
    assert s.d == 1234, "struct values"
    assert s.e == 5991, "struct values"
    assert s.f == U'餐', "struct values"
    assert s.g == somelib.MyEnum.B, "struct values"

    assert s.g == -1, "enum fn"
    assert s.into_a() == 17, "struct fn"
