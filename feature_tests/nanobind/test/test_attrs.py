import somelib

def test_attrs():
    r = somelib.ns.AttrOpaque1Renamed() # Contructor works!
    assert r.method == 77, "property should call"
    assert r.abirenamed == 123, "method should call"

    e = somelib.ns.RenamedAttrEnum.A

    un = somelib.Unnamespaced.make(e)
    un.use_namespaced(r)
    r.use_unnamespaced(un)
    r.use_namespaced(e)

    lst = [1,2,3,4]
    it = somelib.ns.RenamedMyIterable(lst)
    assert len(it) == 4, "Iterable rename failed!"
    lst_copy = [x for x in it]
    assert lst == lst_copy, "Iterable failed!"


    s = somelib.ns.RenamedStructWithAttrs(True, 32) # Just test this doesn't throw
    assert s.b == 32, "Constructor failed!"
    assert s.c == 5, "Getter failed!"

    threw = False
    try:
        s = somelib.RenamedStructWithAttrs(False, 2)
    except Exception:
        threw = True
    assert threw, "Failing constructor should have thrown an error"


    a = somelib.ns.RenamedComparable.new(0)
    b = somelib.ns.RenamedComparable.new(0)
    c = somelib.ns.RenamedComparable.new(1)

    assert a == b, "equality"
    assert b != c, "nequality"
    assert a <= b, "less or equal as equals"
    assert a >= b, "greater or equal as equals"
    assert a <= c, "less or equal"
    assert c >= a, "greater or equal"
    assert a < c, "less"
    assert c > a, "greater"

    assert somelib.ns.RenamedOpaqueArithmetic.make(0, 1).x() == 0
    assert somelib.ns.RenamedOpaqueArithmetic.make(0.5, 1.0).x() == 2
    assert somelib.ns.RenamedStringList.return_new() == ["Test!", 'T', 'e', 's', 't', '!']
    assert somelib.ns.RenamedBlockOverride.special_function() == "This is a custom binding."