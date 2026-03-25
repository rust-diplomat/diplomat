import somelib
import somelib.somelib

import pytest

def test_slices():
    sl = somelib.Float64Vec.new([.1, .2, .3]).asSlice
    b = somelib.Float64Vec.new([.1, .2, .3]).borrow()
    assert all(sl == [.1, .2, .3])
    assert all(b == [.1, .2, .3])

    s = somelib.MyString("banannas").get_static_str()
    b = somelib.MyString("banannas").borrow()
    assert s == "hello"
    assert b == "banannas"
    assert s is not b

    c = somelib.Float64Vec.new([.1, .2, .3])
    d = somelib.Float64VecError.new([.1, .2, .3])
    
    with pytest.raises(IndexError):
        c[4]
        d[4]
    
    l = [somelib.MyString("A"), somelib.MyString(" B "), somelib.MyString("C")]
    assert somelib.MyString.slice_of_opaques(l) == "A B C"

    optional_l = [somelib.MyString("A"), None, somelib.MyString("C")]
    assert somelib.MyString.optional_slice_of_opaques(optional_l) == 'Some(MyString("A")) None Some(MyString("C")) '