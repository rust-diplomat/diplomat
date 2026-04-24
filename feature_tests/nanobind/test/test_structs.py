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

def test_struct_holding_opaques():
    original_op = somelib.Opaque()
    op_st = somelib.StructOfOpaque(original_op, somelib.OpaqueMut())
    assert op_st.i.get_debug_str() == "\"\""
    # Keep alive so it doesn't get garbage collected:
    k = somelib.Opaque.from_str("String")
    op_st.i = k
    assert op_st.i.get_debug_str() == "\"String\""

    immut_st = somelib.ImmutableStructOfOpaque(k)
    assert immut_st.take_in() == "\"String\""

def test_take_in_bound_as_slice():
    a = somelib.PrimitiveStruct(0, True, ' ', 0, 0, 0)
    b = somelib.PrimitiveStruct(10., False, ' ', 0, 0, 0)
    c = somelib.PrimitiveStruct(20., False, ' ', 0, 0, 0)
    l = [a, b, c]
    assert l[0].x == 0
    assert l[1].x == 10
    assert l[2].x == 20

    from_slice = somelib.PrimitiveStructVec.take_in_slice(l)
    assert from_slice[0].x == 0
    assert from_slice[1].x == 10
    assert from_slice[2].x == 20

    from_vec = somelib.PrimitiveStructVec.take_in_slice(from_slice)
    assert from_vec[0].x == 0
    assert from_vec[1].x == 10
    assert from_vec[2].x == 20
