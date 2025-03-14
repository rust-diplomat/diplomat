import somelib

def test_attrs():
    r = somelib.ns.AttrOpaque1Renamed() # Contructor works!
    assert r.method_renamed() == 77, "method should call"
    assert r.abirenamed() == 123, "method should call"

    e = somelib.ns.RenamedAttrEnum.A

    un = somelib.Unnamespaced.make(e)
    un.use_namespaced(r)
    r.use_unnamespaced(un)
    r.use_namespaced(e)

    lst = [1,2,3,4]
    it = somelib.ns.RenamedMyIterable([1,2,3])
    lst_copy = [x for x in it]
    assert lst == lst_copy, "Iterable failed!"