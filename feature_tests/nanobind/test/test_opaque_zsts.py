import somelib
import unittest

class TestOpaqueZST(unittest.TestCase):

    def test_constructor(self):
        a = somelib.ns.RenamedOpaqueZST()
        b = somelib.ns.RenamedOpaqueZST()
        assert(a.out_string == "Test!")
        assert(b.out_string == "Test!")

    def test_static(self):
        a = somelib.ns.RenamedOpaqueZST.make()
        b = somelib.ns.RenamedOpaqueZST.make()
        assert(a.out_string == "Test!")
        assert(b.out_string == "Test!")

    def test_member(self):
        slf = somelib.ns.RenamedOpaqueZST.make()
        a = slf.member()
        b = slf.member()
        assert(slf.out_string == "Test!")
        assert(a.out_string == "Test!")
        assert(b.out_string == "Test!")
    
    def test_mut_member(self):
        slf = somelib.ns.RenamedOpaqueZST.make()
        a = slf.mut_member()
        b = slf.mut_member()
        assert(slf.out_string == "Test!")
        assert(a.out_string == "Test!")
        assert(b.out_string == "Test!")

    def test_success_result(self):
        a = somelib.ns.RenamedOpaqueZST.success_zst(True)
        b = somelib.ns.RenamedOpaqueZST.success_zst(True)
        with self.assertRaises(Exception) as cm:
            somelib.ns.RenamedOpaqueZST.success_zst(False)
        assert(str(cm.exception) == "")

        assert(a.out_string == "Test!")
        assert(b.out_string == "Test!")
    
    def test_fail_result(self):
        c = somelib.ns.RenamedOpaqueZST.fail_zst(True)
        assert(c)

        with self.assertRaises(Exception) as a:
            somelib.ns.RenamedOpaqueZST.fail_zst(False)
            
        with self.assertRaises(Exception) as b:
            somelib.ns.RenamedOpaqueZST.fail_zst(False)
        
        assert(str(a.exception).startswith("<somelib.somelib.ns.RenamedOpaqueZST"))
        assert(str(b.exception).startswith("<somelib.somelib.ns.RenamedOpaqueZST"))
        
    def test_success_fail_result(self):
        a = somelib.ns.RenamedOpaqueZST.success_fail_zst(True)
            
        with self.assertRaises(Exception) as b:
            somelib.ns.RenamedOpaqueZST.success_fail_zst(False)
        
        assert(a.out_string == "Test!")
        assert(str(b.exception).startswith("<somelib.somelib.ns.RenamedOpaqueZST"))
    
    def test_optional(self):
        a = somelib.ns.RenamedOpaqueZST.optional_zst(True)
        b = somelib.ns.RenamedOpaqueZST.optional_zst(True)

        assert(a.out_string == "Test!")
        assert(b.out_string == "Test!")

        c = somelib.ns.RenamedOpaqueZST.optional_zst(False)
        assert(not c)
    
    def test_arithmetic(self):
        a = somelib.ns.RenamedOpaqueZST()
        b = somelib.ns.RenamedOpaqueZST()
        c = a + b
        c1 = a + b
        d = a - b
        d1 = a - b
        e = a * b
        e1 = a * b
        f = a / b
        f1 = a / b
        assert(c.out_string == "Test!")
        assert(c1.out_string == "Test!")
        assert(d.out_string == "Test!")
        assert(d1.out_string == "Test!")
        assert(e.out_string == "Test!")
        assert(e1.out_string == "Test!")
        assert(f.out_string == "Test!")
        assert(f1.out_string == "Test!")

    def test_getters(self):
        a = somelib.ns.RenamedOpaqueZST()
        b = a.getter
        c = a.getter
        assert(b.out_string == "Test!")
        assert(c.out_string == "Test!")

        d = somelib.ns.RenamedOpaqueZST.static_getter
        e = somelib.ns.RenamedOpaqueZST.static_getter
        assert(d.out_string == "Test!")
        assert(e.out_string == "Test!")
    
    def test_iterables(self):
        for j in range(2):
            iteration = 0
            for i in somelib.ns.RenamedOpaqueZST():
                if iteration == 2:
                    break
                iteration += 1
    
    def test_indexer(self):
        a = somelib.ns.RenamedOpaqueZST()
        b = a[0]
        c = a[1]
        assert(b.out_string == "Test!")
        assert(c.out_string == "Test!")

        d = somelib.ns.RenamedOpaqueZSTIterator()
        e = d[0]
        f = d[1]
    
    def test_stringify(self):
        a = somelib.ns.RenamedOpaqueZSTIterator()
        
        with self.assertRaises(Exception) as b:
            str(a)
        with self.assertRaises(Exception) as c:
            str(a)
