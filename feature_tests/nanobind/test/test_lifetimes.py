import somelib
import gc

def test_lifetimes():
    retained_string = "bananna"
    retained_string = retained_string.replace('b', 'f') # modification prevents static storage
    a = somelib.Foo(retained_string)
    retained_string = "bobanna"
    
    gc.collect()

    assert a.as_returning().bytes == "fananna"

    it = somelib.OpaqueThinVec([1,2,3,4], [.1, .2, .3, .4], "")
    for i, o in enumerate(it):
        assert o.a == i+1, "Iteraton over thin vec didn't work"

    a = it.first
    b = it[0]
    
    it = None
    assert a.a == 1 
    assert b.a == 1
    
    del a
    assert b.a == 1

def test_fill_lifetimes():
    
    f = somelib.OpaqueThinVec([120, 2], [.1, .2], "This is a test")
    c = f[0]
    gc.collect()
    
    # Not sure why this works, but this fools the GC into collecting the inner data for f above (even though we still need a reference).
    # We need to check against a string since that takes up more memory than the other fields.
    for i in range(0, 10000):
        f = somelib.OpaqueThinVec([120, 2], [.1, .2], "This is adifferent test")
    gc.collect()

    assert c.c == "This is a test"