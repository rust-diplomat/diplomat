import somelib

def test_callback():
    o = somelib.CallbackWrapper
    global tmp
    tmp = 0
    
    def cb0(a: int) -> int:
        global tmp
        tmp = a
        return a+5
    
    print("test0")

    out = o.test_multi_arg_callback(cb0, 5)
    assert tmp == 15, "multi_arg_callback arg "
    assert out == 20, "multi_arg_callback output"
    
    print("test1")
   
    tmp = 1
    def cb1():
        global tmp
        tmp = tmp+1
    out = o.test_no_args(cb1)
    assert tmp == 2, "test_no_args arg "
    assert out == -5, "test_no_args output"
    print("test3")

    tmp = 0
    def cb2(a):
        global tmp
        tmp = a.y
        return a.x+a.y 
    
    out = o.test_cb_with_struct(cb2)
    assert tmp == 5, "test_cb_with_struct arg "
    assert out == 6, "test_cb_with_struct output"

    print("test4")
    tmp = 0
    global tmp2
    tmp2 = 0
    def cb3():
        global tmp
        tmp = 4
        return 10
    def cb4(a):
        global tmp2
        tmp2 = a
        return a+1
    
    print("DOING OK")
    out = o.test_multiple_cb_args(cb3, cb4)
    assert tmp == 4, "test_multiple_cb_args arg "
    assert tmp2 == 5, "test_multiple_cb_args arg2 "
    assert out == 16, "test_multiple_cb_args output"

    out = o.test_str_cb_arg(lambda a: len(a))
    assert out == 7, "test_str_cb_arg output"
    print("END")
