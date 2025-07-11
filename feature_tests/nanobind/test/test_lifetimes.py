import somelib
import gc

def test_lifetimes():
    retained_string = "bananna"
    retained_string = retained_string.replace('b', 'f') # modification prevents static storage
    a = somelib.Foo(retained_string)
    retained_string = "bobanna"
    
    gc.collect()

    assert a.as_returning().bytes == "fananna"

    it = somelib.OpaqueThinVec([1,2,3,4], [.1, .2, .3, .4])
    for i, o in enumerate(it):
        assert o.a == i+1, "Iteraton over thin vec didn't work"

    a = it.first
    b = it[0]
    
    it = None
    assert a.a == 1 
    assert b.a == 1
    
    del a
    assert b.a == 1
