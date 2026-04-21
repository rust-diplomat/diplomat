import somelib
def test_structs():
    o = somelib.Opaque()
    s = somelib.MyStruct()

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

    
    s2 = somelib.MyStruct(10)
    assert s2.e == 10

    assert somelib.StructArithmetic.ORIGIN.x == 0

    sl = somelib.PrimitiveStructVec()
    sl.append(somelib.PrimitiveStruct(1, True, 'c', 0, 0, 0))
    sl.append(somelib.PrimitiveStruct(2, False, ' ', 0, 0, 0))
    sl.append(somelib.PrimitiveStruct(-1, False, ' ', 0, 0, 0))
    sl = sl.asSlice
    assert sl[0].x == 1
    assert sl[1].x == 2
    assert sl[2].x == -1

    bg = somelib.BigStructWithStuffSlice()
    bg.append(somelib.BigStructWithStuff())
    bg.append(somelib.BigStructWithStuff(1, 2, 3, somelib.ScalarPairWithPadding(1, 2), 0))
    somelib.BigStructWithStuff.assert_slice(bg, 2)

    original_op = somelib.Opaque()
    op_st = somelib.StructOfOpaque(original_op, somelib.OpaqueMut())
    assert op_st.i.get_debug_str() == "\"\""
    # Keep alive so it doesn't get garbage collected:
    k = somelib.Opaque.from_str("String")
    op_st.i = k
    assert op_st.i.get_debug_str() == "\"String\""