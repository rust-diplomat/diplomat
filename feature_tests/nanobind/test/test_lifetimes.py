import somelib
import gc

def test_lifetimes():
    retained_string = "bananna"
    retained_string = retained_string.replace('b', 'f') # modification prevents static storage
    a = somelib.Foo(retained_string)
    retained_string = "bobanna"
    
    gc.collect()

    assert a.as_returning().bytes == "fananna"
