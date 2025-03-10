import somelib

def test_attrs():
    r = somelib.ns.AttrOpaque1Renamed.totally_not_new()
    assert r.method_renamed() == 77, "method should call"
    assert r.abirenamed() == 123, "method should call"

    e = somelib.ns.RenamedAttrEnum.A

    un = somelib.Unnamespaced.make(e)
    un.use_namespaced(r)
    r.use_unnamespaced(un)
    r.use_namespaced(e)
