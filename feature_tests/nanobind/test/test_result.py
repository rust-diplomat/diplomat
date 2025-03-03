import somelib
import unittest

class TestResult(unittest.TestCase):
    
    def test_result(self):
        r2 = somelib.ResultOpaque.new_(5)
        r2.assert_integer(5)

        with self.assertRaises(Exception) as cm:
            somelib.ResultOpaque.new_failing_foo()

        assert str(cm.exception) == "ErrorEnum.Foo", "foo error"

        with self.assertRaises(Exception) as cm:
            somelib.ResultOpaque.new_failing_bar()
        assert str(cm.exception) == str(somelib.ErrorEnum.Bar), "bar error"

        with self.assertRaises(Exception) as cm:
            somelib.ResultOpaque.new_failing_unit()
        assert cm.exception, "unit error"

        with self.assertRaises(Exception) as cm:
            somelib.ResultOpaque.new_failing_struct(109)
        # Exceptions are universally converted to strings, so we cannot access fields.
        # If present, stringifiers are used
        assert str(cm.exception).startswith("<somelib.somelib.ErrorStruct object"), "struct error"

        integer = somelib.ResultOpaque.new_int(109)
        assert integer == 109, "int ok"

        with self.assertRaises(Exception) as cm:
            somelib.ResultOpaque.new_in_err(198)
        # No methods allowed, exceptions are only strings
        #cm.exception.assert_integer(198)
        assert(str(cm.exception).startswith("<somelib.somelib.ResultOpaque"))


