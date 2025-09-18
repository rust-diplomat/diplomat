import somelib

def test_free_functions():
    assert somelib.nested.ns.Renamednested_ns_fn(True) == False
    assert somelib.ns.Renamedfree_func_test(0) == 5