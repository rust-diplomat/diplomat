import somelib
import unittest

class TestResult(unittest.TestCase):
    
    def test_result(self):
        r2 = somelib.ResultOpaque(5)
        r2.assert_integer(5)

        with self.assertRaises(Exception) as cm:
            somelib.ResultOpaque.new_failing_foo()

        assert cm.exception.args[0] == somelib.ErrorEnum.Foo, "foo error"

        with self.assertRaises(Exception) as cm:
            somelib.ResultOpaque.new_failing_bar()
        assert cm.exception.args[0] == somelib.ErrorEnum.Bar, "bar error"

        with self.assertRaises(Exception) as cm:
            somelib.ResultOpaque.new_failing_unit()
        assert cm.exception, "unit error"

        with self.assertRaises(Exception) as cm:
            somelib.ResultOpaque.new_failing_struct(109)
        # Exceptions are universally converted to strings, so we cannot access fields.
        # If present, stringifiers are used
        assert cm.exception.args[0].j == 12, "struct error"
        assert cm.exception.args[0].i == 109, "struct error"

        integer = somelib.ResultOpaque.new_int(109)
        assert integer == 109, "int ok"

        with self.assertRaises(Exception) as cm:
            somelib.ResultOpaque.new_in_err(198)
        # No methods allowed, exceptions are only strings
        #cm.exception.assert_integer(198)
        cm.exception.args[0].assert_integer(198)

        a = somelib.ResultOpaque(102)
        b = a.takes_str("Hello there")
        a.assert_integer(102)
        b.assert_integer(102)

        with self.assertRaises(Exception) as cm:
            a = somelib.ResultOpaque(0)
            a.give_self()
        a.assert_integer(0)

    def test_stringifier_loop(self):
        a = somelib.ResultOpaque(0)
        with self.assertRaises(Exception) as cm:
            str(a)
        
        cm.exception.args[0].assert_integer(0)
        
        with self.assertRaises(Exception) as cm2:
            str(cm.exception)
        cm2.exception.args[0].assert_integer(0)
