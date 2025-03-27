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
        s = somelib.RenamedStructWithAttrs(2)
    except Exception:
        threw = True
    assert threw, "Failing constructor should have thrown an error"
