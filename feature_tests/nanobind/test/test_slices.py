import somelib
import somelib.somelib
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
