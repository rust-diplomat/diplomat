import somelib
import pytest

class ImplementsTrait(somelib.FallibleTesterTrait):
    def __init__(self):
        super().__init__()
        self.a = 0

    def test_void_trait_fn(self):
        self.a = "Test!"

    def test_result_output(self, x : int):
        if x == 42:
            return "Invalid"
        elif x == 28:
            return somelib.FFIError(somelib.FFIError.User)
        elif x == 31:
            raise Exception("Some exception!")
        return x

class Unimplemented(somelib.FallibleTesterTrait):
    pass

def test_unimplemented_excepts():
    with pytest.raises(AttributeError):
        u = Unimplemented()

class WrongBase(somelib.FallibleTesterTrait_PyBase):
    pass

def test_wrong_base_extension():
    with pytest.raises(TypeError):
        w = WrongBase()

class SupersIncorrectly(somelib.FallibleTesterTrait):
    def test_result_output(x):
        return super().test_result_output(x)

    def test_void_trait_fn():
        pass

def test_supers_incorrectly():
    with pytest.raises(Exception) as e:
        somelib.FallibleTraitWrapper.test_result_output(SupersIncorrectly(), 0)
    # Currently there's not a way to check if a user super() calls the function (even though they shouldn't).
    assert e.value.args[0] == somelib.FFIError.FFI

def test_prim_trait_fn():
    i = ImplementsTrait()
    assert i.a == 0
    somelib.FallibleTraitWrapper.test_with_trait(i)
    assert i.a == "Test!"

# Nanobind will warn about the destruction, we ignore that here.
@pytest.mark.filterwarnings("ignore:nanobind")
def test_trait_destroyed():
    i = ImplementsTrait()
    somelib.FallibleTraitWrapper.test_with_trait(i)

    with pytest.raises(TypeError):
        somelib.FallibleTraitWrapper.test_with_trait(i)

def test_result_trait():
    i = ImplementsTrait()
    assert somelib.FallibleTraitWrapper.test_result_output(i, 5) == 5

    i = ImplementsTrait()
    # Returning an invalid type:
    with pytest.raises(Exception) as e:
        somelib.FallibleTraitWrapper.test_result_output(i, 42)
    assert e.value.args[0] == somelib.FFIError.FFI
    ffi_error = somelib.get_last_ffi_error()
    assert type(ffi_error) == TypeError
    assert ffi_error.args[0] == "Unable to convert trait return type"

    i = ImplementsTrait()
    # Other result types still work:
    with pytest.raises(Exception) as e:
        somelib.FallibleTraitWrapper.test_result_output(i, 28)
    assert e.value.args[0] == somelib.FFIError.User

    # Test exception raising returns FFI Error:
    i = ImplementsTrait()
    with pytest.raises(Exception) as e:
        somelib.FallibleTraitWrapper.test_result_output(i, 31)
    assert e.value.args[0] == somelib.FFIError.FFI
    ffi_error = somelib.get_last_ffi_error()
    assert type(ffi_error) == Exception
    assert ffi_error.args[0] == "Some exception!"
