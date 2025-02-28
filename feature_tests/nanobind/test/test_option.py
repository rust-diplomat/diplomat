import somelib
import typing

def test_option():
    o = somelib.OptionOpaque.new_(1415)
    o.assert_integer(1415)

    o = somelib.OptionOpaque.new_none()
    assert not o, "new_none() returns None"

    s = somelib.OptionOpaque.new_struct()
    s.a.assert_integer(101)
    s.b.assert_char(U'餐')
    assert s.c == 904, "correct struct returned"
    s.d.assert_integer(926535)

    s = somelib.OptionOpaque.new_struct_nones()

    assert not s.a, "new_struct_nones() returns None"
    assert not s.b, "new_struct_nones() returns None"
    assert s.c == 908, "correct struct returned"

    opt_u8 = somelib.OptionOpaque.accepts_option_u8(None)
    assert not opt_u8, "accepts_option_u8 is idempotent"
    opt_u8 = somelib.OptionOpaque.accepts_option_u8(5)
    assert opt_u8 == 5, "accepts_option_u8 is idempotent"
    opt_enum = somelib.OptionOpaque.accepts_option_enum(None)
    assert not opt_enum, "accepts_option_enum is idempotent"
    opt_enum = somelib.OptionOpaque.accepts_option_enum(somelib.OptionEnum.Foo)
    assert opt_enum == somelib.OptionEnum.Foo, "accepts_option_enum is idempotent"
    opt_struct = somelib.OptionOpaque.accepts_option_input_struct(None)
    assert not opt_struct, "accepts_option_input_struct is idempotent"
    opt_struct = somelib.OptionOpaque.accepts_option_input_struct(somelib.OptionInputStruct(1, None, somelib.OptionEnum.Foo))
    assert opt_struct.a == 1, "accepts_option_input_struct is idempotent"
    assert not opt_struct.b, "accepts_option_input_struct is idempotent"
    assert opt_struct.c == somelib.OptionEnum.Foo, "accepts_option_input_struct is idempotent"

    opt_struct = somelib.OptionOpaque.returns_option_input_struct()
    assert opt_struct.a == 6, "returns_option_input_struct returns the right values"
    assert not opt_struct.b, "returns_option_input_struct returns the right values"
    assert opt_struct.c == somelib.OptionEnum.Bar, "returns_option_input_struct returns the right values"
