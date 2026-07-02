from collections.abc import Callable, Iterable, Iterator, Sequence
import enum
from typing import overload

from . import mylib as mylib, nested as nested, ns as ns


class Bar:
    @property
    def foo(self) -> Foo: ...

class BigStructWithStuff:
    """
    Testing JS-specific layout/padding behavior
    Also being used to test CPP backends taking structs with primitive values.
    """

    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, first: int | None, second: int | None, third: int | None, fourth: ScalarPairWithPadding | None, fifth: int | None) -> None: ...

    @property
    def first(self) -> int: ...

    @first.setter
    def first(self, arg: int, /) -> None: ...

    @property
    def second(self) -> int: ...

    @second.setter
    def second(self, arg: int, /) -> None: ...

    @property
    def third(self) -> int: ...

    @third.setter
    def third(self, arg: int, /) -> None: ...

    @property
    def fourth(self) -> ScalarPairWithPadding: ...

    @fourth.setter
    def fourth(self, arg: ScalarPairWithPadding, /) -> None: ...

    @property
    def fifth(self) -> int: ...

    @fifth.setter
    def fifth(self, arg: int, /) -> None: ...

    @staticmethod
    def assert_slice(slice: Sequence[BigStructWithStuff], second_value: int) -> None: ...

    def assert_value(self, extra_val: int) -> None: ...

class BigStructWithStuffSlice:
    @overload
    def __init__(self) -> None:
        """Default constructor"""

    @overload
    def __init__(self, arg: BigStructWithStuffSlice) -> None:
        """Copy constructor"""

    @overload
    def __init__(self, arg: Iterable[BigStructWithStuff], /) -> None:
        """Construct from an iterable object"""

    def __len__(self) -> int: ...

    def __bool__(self) -> bool:
        """Check whether the vector is nonempty"""

    def __repr__(self) -> str: ...

    def __iter__(self) -> Iterator[BigStructWithStuff]: ...

    @overload
    def __getitem__(self, arg: int, /) -> BigStructWithStuff: ...

    @overload
    def __getitem__(self, arg: slice, /) -> BigStructWithStuffSlice: ...

    def clear(self) -> None:
        """Remove all items from list."""

    def append(self, arg: BigStructWithStuff, /) -> None:
        """Append `arg` to the end of the list."""

    def insert(self, arg0: int, arg1: BigStructWithStuff, /) -> None:
        """Insert object `arg1` before index `arg0`."""

    def pop(self, index: int = -1) -> BigStructWithStuff:
        """Remove and return item at `index` (default last)."""

    def extend(self, arg: BigStructWithStuffSlice, /) -> None:
        """Extend `self` by appending elements from `arg`."""

    @overload
    def __setitem__(self, arg0: int, arg1: BigStructWithStuff, /) -> None: ...

    @overload
    def __setitem__(self, arg0: slice, arg1: BigStructWithStuffSlice, /) -> None: ...

    @overload
    def __delitem__(self, arg: int, /) -> None: ...

    @overload
    def __delitem__(self, arg: slice, /) -> None: ...

class BorrowedFields:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, a: "std::basic_string_view<char16_t, std::char_traits<char16_t> >" | None, b: str | None, c: str | None) -> None: ...

    @property
    def a(self) -> "std::basic_string_view<char16_t, std::char_traits<char16_t> >": ...

    @a.setter
    def a(self, arg: "std::basic_string_view<char16_t, std::char_traits<char16_t> >", /) -> None: ...

    @property
    def b(self) -> str: ...

    @b.setter
    def b(self, arg: str, /) -> None: ...

    @property
    def c(self) -> str: ...

    @c.setter
    def c(self, arg: str, /) -> None: ...

    @staticmethod
    def from_bar_and_strings(bar: Bar, dstr16: "std::basic_string_view<char16_t, std::char_traits<char16_t> >", utf8_str: str) -> BorrowedFields: ...

class BorrowedFieldsReturning:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, bytes: str | None) -> None: ...

    @property
    def bytes(self) -> str: ...

    @bytes.setter
    def bytes(self, arg: str, /) -> None: ...

class BorrowedFieldsWithBounds:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, field_a: "std::basic_string_view<char16_t, std::char_traits<char16_t> >" | None, field_b: str | None, field_c: str | None) -> None: ...

    @property
    def field_a(self) -> "std::basic_string_view<char16_t, std::char_traits<char16_t> >": ...

    @field_a.setter
    def field_a(self, arg: "std::basic_string_view<char16_t, std::char_traits<char16_t> >", /) -> None: ...

    @property
    def field_b(self) -> str: ...

    @field_b.setter
    def field_b(self, arg: str, /) -> None: ...

    @property
    def field_c(self) -> str: ...

    @field_c.setter
    def field_c(self, arg: str, /) -> None: ...

    @staticmethod
    def from_foo_and_strings(foo: Foo, dstr16_x: "std::basic_string_view<char16_t, std::char_traits<char16_t> >", utf8_str_z: str) -> BorrowedFieldsWithBounds: ...

class BorrowingOptionStruct:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, a: str | None) -> None: ...

    @property
    def a(self) -> str | None: ...

    @a.setter
    def a(self, arg: str, /) -> None: ...

class CachedIncludeZST:
    def __init__(self) -> None: ...

class CallbackHolder:
    def __init__(self, func: Callable[[int], int]) -> None: ...

    def call(self, a: int) -> int: ...

class CallbackTestingStruct:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, x: int | None, y: int | None) -> None: ...

    @property
    def x(self) -> int: ...

    @x.setter
    def x(self, arg: int, /) -> None: ...

    @property
    def y(self) -> int: ...

    @y.setter
    def y(self, arg: int, /) -> None: ...

class CallbackWrapper:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, cant_be_empty: bool | None) -> None: ...

    @property
    def cant_be_empty(self) -> bool: ...

    @cant_be_empty.setter
    def cant_be_empty(self, arg: bool, /) -> None: ...

    @staticmethod
    def test_cb_with_struct(f: Callable[[CallbackTestingStruct], int]) -> int: ...

    @staticmethod
    def test_diplomat_option_output(t: Callable[[], int | None]) -> None: ...

    @staticmethod
    def test_diplomat_result(t: Callable[[], int]) -> None: ...

    @staticmethod
    def test_inner_conversion(t: Callable[[], MyStructContainingAnOption]) -> None: ...

    @staticmethod
    def test_multi_arg_callback(f: Callable[[int], int], x: int) -> int: ...

    @staticmethod
    def test_multiple_cb_args(f: Callable[[], int], g: Callable[[int], int]) -> int: ...

    @staticmethod
    def test_no_args(h: Callable[[], None]) -> int: ...

    @staticmethod
    def test_opaque_cb_arg(cb: Callable[[MyString], None], a: MyString) -> None: ...

    @staticmethod
    def test_opaque_result_error(t: Callable[[], None]) -> str: ...

    @staticmethod
    def test_option_opaque(t: Callable[[], Opaque]) -> str: ...

    @staticmethod
    def test_option_output(t: Callable[[], monostate | None]) -> None: ...

    @staticmethod
    def test_owned_opaque(t: Callable[[Opaque], None]) -> None: ...

    @staticmethod
    def test_result_opaque(t: Callable[[], Opaque]) -> str: ...

    @staticmethod
    def test_result_option_struct_conversion(t: Callable[[], MyStruct | None]) -> None: ...

    @staticmethod
    def test_result_output(t: Callable[[], None]) -> None: ...

    @staticmethod
    def test_result_usize_output(t: Callable[[], int]) -> None: ...

    @staticmethod
    def test_slice_cb_arg(arg: Sequence[int], f: Callable[[Sequence[int]], None]) -> None: ...

    @staticmethod
    def test_slice_conversion(t: Callable[[], Sequence[float]]) -> None: ...

    @staticmethod
    def test_str_cb_arg(f: Callable[[str], int]) -> int: ...

    @staticmethod
    def test_str_conversion(t: Callable[[], str]) -> None: ...

    @staticmethod
    def test_struct_slice_conversion(t: Callable[[], Sequence[PrimitiveStruct]]) -> None: ...

class ContainingTuple:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, inner: tuple[int, int, MyStruct, Opaque] | None) -> None: ...

    @property
    def inner(self) -> tuple[int, int, MyStruct, Opaque]: ...

    @inner.setter
    def inner(self, arg: tuple[int, int, MyStruct, Opaque], /) -> None: ...

class ContiguousEnum:
    def __init__(self, arg: ContiguousEnum.ContiguousEnum, /) -> None: ...

    class ContiguousEnum(enum.Enum):
        C = 0

        D = 1

        E = 2

        F = 3

    C: ContiguousEnum.ContiguousEnum = ContiguousEnum.ContiguousEnum.C

    D: ContiguousEnum.ContiguousEnum = ContiguousEnum.ContiguousEnum.D

    E: ContiguousEnum.ContiguousEnum = ContiguousEnum.ContiguousEnum.E

    F: ContiguousEnum.ContiguousEnum = ContiguousEnum.ContiguousEnum.F

    def __eq__(self, arg: ContiguousEnum.ContiguousEnum, /) -> bool: ...

    def __repr__(self) -> str: ...

class CyclicStructA:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, a: CyclicStructB | None) -> None: ...

    @property
    def a(self) -> CyclicStructB: ...

    @a.setter
    def a(self, arg: CyclicStructB, /) -> None: ...

    def cyclic_out(self) -> str: ...

    def double_cyclic_out(self, cyclic_struct_a: CyclicStructA) -> str: ...

    @staticmethod
    def get_b() -> CyclicStructB: ...

    @property
    def getter_out(self) -> str: ...

    @staticmethod
    def nested_slice(sl: Sequence[CyclicStructA]) -> int: ...

class CyclicStructASlice:
    @overload
    def __init__(self) -> None:
        """Default constructor"""

    @overload
    def __init__(self, arg: CyclicStructASlice) -> None:
        """Copy constructor"""

    @overload
    def __init__(self, arg: Iterable[CyclicStructA], /) -> None:
        """Construct from an iterable object"""

    def __len__(self) -> int: ...

    def __bool__(self) -> bool:
        """Check whether the vector is nonempty"""

    def __repr__(self) -> str: ...

    def __iter__(self) -> Iterator[CyclicStructA]: ...

    @overload
    def __getitem__(self, arg: int, /) -> CyclicStructA: ...

    @overload
    def __getitem__(self, arg: slice, /) -> CyclicStructASlice: ...

    def clear(self) -> None:
        """Remove all items from list."""

    def append(self, arg: CyclicStructA, /) -> None:
        """Append `arg` to the end of the list."""

    def insert(self, arg0: int, arg1: CyclicStructA, /) -> None:
        """Insert object `arg1` before index `arg0`."""

    def pop(self, index: int = -1) -> CyclicStructA:
        """Remove and return item at `index` (default last)."""

    def extend(self, arg: CyclicStructASlice, /) -> None:
        """Extend `self` by appending elements from `arg`."""

    @overload
    def __setitem__(self, arg0: int, arg1: CyclicStructA, /) -> None: ...

    @overload
    def __setitem__(self, arg0: slice, arg1: CyclicStructASlice, /) -> None: ...

    @overload
    def __delitem__(self, arg: int, /) -> None: ...

    @overload
    def __delitem__(self, arg: slice, /) -> None: ...

class CyclicStructB:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, field: int | None) -> None: ...

    @property
    def field(self) -> int: ...

    @field.setter
    def field(self, arg: int, /) -> None: ...

    @staticmethod
    def get_a() -> CyclicStructA: ...

    @staticmethod
    def get_a_option() -> CyclicStructA | None: ...

class CyclicStructBSlice:
    @overload
    def __init__(self) -> None:
        """Default constructor"""

    @overload
    def __init__(self, arg: CyclicStructBSlice) -> None:
        """Copy constructor"""

    @overload
    def __init__(self, arg: Iterable[CyclicStructB], /) -> None:
        """Construct from an iterable object"""

    def __len__(self) -> int: ...

    def __bool__(self) -> bool:
        """Check whether the vector is nonempty"""

    def __repr__(self) -> str: ...

    def __iter__(self) -> Iterator[CyclicStructB]: ...

    @overload
    def __getitem__(self, arg: int, /) -> CyclicStructB: ...

    @overload
    def __getitem__(self, arg: slice, /) -> CyclicStructBSlice: ...

    def clear(self) -> None:
        """Remove all items from list."""

    def append(self, arg: CyclicStructB, /) -> None:
        """Append `arg` to the end of the list."""

    def insert(self, arg0: int, arg1: CyclicStructB, /) -> None:
        """Insert object `arg1` before index `arg0`."""

    def pop(self, index: int = -1) -> CyclicStructB:
        """Remove and return item at `index` (default last)."""

    def extend(self, arg: CyclicStructBSlice, /) -> None:
        """Extend `self` by appending elements from `arg`."""

    @overload
    def __setitem__(self, arg0: int, arg1: CyclicStructB, /) -> None: ...

    @overload
    def __setitem__(self, arg0: slice, arg1: CyclicStructBSlice, /) -> None: ...

    @overload
    def __delitem__(self, arg: int, /) -> None: ...

    @overload
    def __delitem__(self, arg: slice, /) -> None: ...

class CyclicStructC:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, a: CyclicStructA | None) -> None: ...

    @property
    def a(self) -> CyclicStructA: ...

    @a.setter
    def a(self, arg: CyclicStructA, /) -> None: ...

    def cyclic_out(self) -> str: ...

    @staticmethod
    def takes_nested_parameters(c: CyclicStructC) -> CyclicStructC: ...

class DefaultEnum:
    def __init__(self, arg: DefaultEnum.DefaultEnum, /) -> None: ...

    class DefaultEnum(enum.Enum):
        A = 0

        B = 1

    A: DefaultEnum.DefaultEnum = DefaultEnum.DefaultEnum.A

    B: DefaultEnum.DefaultEnum = DefaultEnum.DefaultEnum.B

    def __eq__(self, arg: DefaultEnum.DefaultEnum, /) -> bool: ...

    def __repr__(self) -> str: ...

class ErrorEnum:
    def __init__(self, arg: ErrorEnum.ErrorEnum, /) -> None: ...

    class ErrorEnum(enum.Enum):
        Foo = 0

        Bar = 1

    Foo: ErrorEnum.ErrorEnum = ErrorEnum.ErrorEnum.Foo

    Bar: ErrorEnum.ErrorEnum = ErrorEnum.ErrorEnum.Bar

    def __eq__(self, arg: ErrorEnum.ErrorEnum, /) -> bool: ...

    def __repr__(self) -> str: ...

class ErrorStruct:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, i: int | None, j: int | None) -> None: ...

    @property
    def i(self) -> int: ...

    @i.setter
    def i(self, arg: int, /) -> None: ...

    @property
    def j(self) -> int: ...

    @j.setter
    def j(self, arg: int, /) -> None: ...

    @staticmethod
    def returns_result_option(is_some: bool) -> ErrorStruct | None: ...

class FallibleOpaqueConstructor:
    def __init__(self) -> None: ...

    @property
    def x(self) -> int: ...

    @x.setter
    def x(self, arg: int, /) -> None: ...

class Float64Vec:
    @property
    def asSlice(self) -> list[float]: ...

    def borrow(self) -> list[float]: ...

    def __getitem__(self, i: int) -> float | None: ...

    @staticmethod
    def new(v: Sequence[float]) -> Float64Vec: ...

    @staticmethod
    def new_bool(v: Sequence[bool]) -> Float64Vec: ...

    @staticmethod
    def new_f64_be_bytes(v: Sequence[int]) -> Float64Vec: ...

    @staticmethod
    def new_i16(v: Sequence[int]) -> Float64Vec: ...

    @staticmethod
    def new_isize(v: Sequence[int]) -> Float64Vec: ...

    @staticmethod
    def new_u16(v: Sequence[int]) -> Float64Vec: ...

    @staticmethod
    def new_usize(v: Sequence[int]) -> Float64Vec: ...

    def set_value(self, new_slice: Sequence[float]) -> None: ...

    def __str__(self) -> str: ...

class Float64VecError:
    def __getitem__(self, i: int) -> float: ...

    @staticmethod
    def new(v: Sequence[float]) -> Float64VecError: ...

class Foo:
    def __init__(self, x: str) -> None: ...

    def as_returning(self) -> BorrowedFieldsReturning: ...

    @property
    def bar(self) -> Bar: ...

    @staticmethod
    def extract_from_bounds(bounds: BorrowedFieldsWithBounds, another_string: str) -> Foo:
        """Test that the extraction logic correctly pins the right fields"""

    @staticmethod
    def extract_from_fields(fields: BorrowedFields) -> Foo: ...

    @staticmethod
    def new_static(x: str) -> Foo: ...

class ImmutableStructOfOpaque:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, i: Opaque | None) -> None: ...

    @property
    def i(self) -> Opaque: ...

    @i.setter
    def i(self, arg: Opaque, /) -> None: ...

    def take_in(self) -> str: ...

class ImportedStruct:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, foo: UnimportedEnum | None, count: int | None) -> None: ...

    @property
    def foo(self) -> UnimportedEnum: ...

    @foo.setter
    def foo(self, arg: UnimportedEnum, /) -> None: ...

    @property
    def count(self) -> int: ...

    @count.setter
    def count(self, arg: int, /) -> None: ...

class MutableCallbackHolder:
    def __init__(self, func: Callable[[int], int]) -> None: ...

    def call(self, a: int) -> int: ...

    def opaque_cb_mut_self(self, cb: Callable[[MyString], None], st: MyString) -> None: ...

    def opaque_cb_self(self, cb: Callable[[MyString], None], st: MyString) -> None: ...

class MyEnum:
    def __init__(self, arg: MyEnum.MyEnum, /) -> None: ...

    class MyEnum(enum.Enum):
        A = -2

        B = -1
        """.. deprecated:: C is the new B"""

        C = 0

        D = 1

        E = 2
        """EEEEEEE"""

        F = 3

    A: MyEnum.MyEnum = MyEnum.MyEnum.A

    B: MyEnum.MyEnum = MyEnum.MyEnum.B

    C: MyEnum.MyEnum = MyEnum.MyEnum.C

    D: MyEnum.MyEnum = MyEnum.MyEnum.D

    E: MyEnum.MyEnum = MyEnum.MyEnum.E

    F: MyEnum.MyEnum = MyEnum.MyEnum.F

    def __eq__(self, arg: MyEnum.MyEnum, /) -> bool: ...

    def __repr__(self) -> str: ...

class MyOpaqueEnum:
    @staticmethod
    def new() -> MyOpaqueEnum: ...

    def __str__(self) -> str: ...

class MyString:
    def __init__(self, v: str = 'T') -> None: ...

    def borrow(self) -> str: ...

    @staticmethod
    def get_static_str() -> str: ...

    @staticmethod
    def new_from_first(v: Sequence[str]) -> MyString: ...

    @staticmethod
    def new_from_utf16(v: Sequence["somelib::diplomat::basic_string_view_for_slice<char16_t, std::char_traits<char16_t> >"]) -> MyString: ...

    @staticmethod
    def new_unsafe(v: str) -> MyString: ...

    @staticmethod
    def optional_slice_of_opaques(sl: Sequence[MyString]) -> str: ...

    @staticmethod
    def other_opaque_type(other: Sequence[Float64Vec]) -> str: ...

    @staticmethod
    def slice_of_opaques(sl: Sequence[MyString]) -> str: ...

    @property
    def str(self) -> str: ...

    @str.setter
    def str(self, arg: str, /) -> None: ...

    @staticmethod
    def string_transform(foo: str) -> str: ...

class MyStruct:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, i: int) -> None: ...

    @property
    def a(self) -> int: ...

    @a.setter
    def a(self, arg: int, /) -> None: ...

    @property
    def b(self) -> bool: ...

    @b.setter
    def b(self, arg: bool, /) -> None: ...

    @property
    def c(self) -> int: ...

    @c.setter
    def c(self, arg: int, /) -> None: ...

    @property
    def d(self) -> int: ...

    @d.setter
    def d(self, arg: int, /) -> None: ...

    @property
    def e(self) -> int: ...

    @e.setter
    def e(self, arg: int, /) -> None: ...

    @property
    def f(self) -> str: ...

    @f.setter
    def f(self, arg: str, /) -> None: ...

    @property
    def g(self) -> MyEnum: ...

    @g.setter
    def g(self, arg: MyEnum, /) -> None: ...

    @staticmethod
    def fails_zst_result() -> None: ...

    def into_a(self) -> int: ...

    @staticmethod
    def returns_zst_result() -> None: ...

    def take_ref_ret(self) -> int: ...

    def takes_const(self, o: MyStruct) -> None: ...

    def takes_mut(self, o: MyStruct) -> None: ...

class MyStructContainingAnOption:
    def __init__(self) -> None: ...

    @property
    def a(self) -> MyStruct | None: ...

    @a.setter
    def a(self, arg: MyStruct, /) -> None: ...

    @property
    def b(self) -> DefaultEnum | None: ...

    @b.setter
    def b(self, arg: DefaultEnum, /) -> None: ...

    @staticmethod
    def filled() -> MyStructContainingAnOption: ...

class MyZst:
    def __init__(self) -> None: ...

class NestedBorrowedFields:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, fields: BorrowedFields | None, bounds: BorrowedFieldsWithBounds | None, bounds2: BorrowedFieldsWithBounds | None) -> None: ...

    @property
    def fields(self) -> BorrowedFields: ...

    @fields.setter
    def fields(self, arg: BorrowedFields, /) -> None: ...

    @property
    def bounds(self) -> BorrowedFieldsWithBounds: ...

    @bounds.setter
    def bounds(self, arg: BorrowedFieldsWithBounds, /) -> None: ...

    @property
    def bounds2(self) -> BorrowedFieldsWithBounds: ...

    @bounds2.setter
    def bounds2(self, arg: BorrowedFieldsWithBounds, /) -> None: ...

    @staticmethod
    def from_bar_and_foo_and_strings(bar: Bar, foo: Foo, dstr16_x: "std::basic_string_view<char16_t, std::char_traits<char16_t> >", dstr16_z: "std::basic_string_view<char16_t, std::char_traits<char16_t> >", utf8_str_y: str, utf8_str_z: str) -> NestedBorrowedFields: ...

class One:
    @staticmethod
    def cycle(hold: Two, nohold: One) -> One: ...

    @staticmethod
    def diamond_and_nested_types(a: One, b: One, c: One, d: One, nohold: One) -> One: ...

    @staticmethod
    def diamond_bottom(top: One, left: One, right: One, bottom: One) -> One: ...

    @staticmethod
    def diamond_left(top: One, left: One, right: One, bottom: One) -> One: ...

    @staticmethod
    def diamond_right(top: One, left: One, right: One, bottom: One) -> One: ...

    @staticmethod
    def diamond_top(top: One, left: One, right: One, bottom: One) -> One: ...

    @staticmethod
    def implicit_bounds(explicit_hold: One, implicit_hold: One, nohold: One) -> One: ...

    @staticmethod
    def implicit_bounds_deep(explicit_: One, implicit_1: One, implicit_2: One, nohold: One) -> One: ...

    @staticmethod
    def many_dependents(a: One, b: One, c: Two, d: Two, nohold: Two) -> One: ...

    @staticmethod
    def return_outlives_param(hold: Two, nohold: One) -> One: ...

    @staticmethod
    def transitivity(hold: One, nohold: One) -> One: ...

class Opaque:
    def __init__(self) -> None: ...

    def assert_struct(self, s: MyStruct) -> None:
        """
        See the [Rust documentation for `something`](https://docs.rs/Something/latest/struct.Something.html#method.something) for more information.

        See the [Rust documentation for `something_else`](https://docs.rs/Something/latest/struct.Something.html#method.something_else) for more information.

        Additional information: [1](https://docs.rs/Something/latest/struct.Something.html#method.something_small), [2](https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something)
        """

    @staticmethod
    def cmp() -> int: ...

    @staticmethod
    def from_str(input: str) -> Opaque: ...

    def get_debug_str(self) -> str: ...

    @staticmethod
    def returns_imported() -> ImportedStruct: ...

    @staticmethod
    def returns_usize() -> int: ...

    @staticmethod
    def try_from_utf8(input: str) -> Opaque: ...

class OpaqueCallbacks:
    @staticmethod
    def ret_op(f: Callable[[MyString], MyString], st: MyString) -> MyString: ...

class OpaqueMut:
    def __init__(self) -> None: ...

class OpaqueMutexedString:
    def borrow(self) -> OpaqueMutexedString: ...

    @staticmethod
    def borrow_other(other: OpaqueMutexedString) -> OpaqueMutexedString: ...

    def borrow_self_or_other(self, other: OpaqueMutexedString) -> OpaqueMutexedString: ...

    def change(self, number: int) -> None: ...

    def dummy_str(self) -> str: ...

    @staticmethod
    def from_usize(number: int) -> OpaqueMutexedString: ...

    def get_len_and_add(self, other: int) -> int: ...

    def to_unsigned_from_unsigned(self, input: int) -> int: ...

    def wrapper(self) -> Utf16Wrap: ...

class OpaqueThin:
    @property
    def a(self) -> int: ...

    @property
    def b(self) -> float: ...

    @property
    def c(self) -> str: ...

class OpaqueThinIter:
    def __next__(self) -> OpaqueThin: ...

    def __iter__(self) -> object: ...

class OpaqueThinVec:
    def __init__(self, a: Sequence[int], b: Sequence[float], c: str) -> None: ...

    def __len__(self) -> int: ...

    @property
    def first(self) -> OpaqueThin: ...

    def __getitem__(self, idx: int) -> OpaqueThin: ...

    def __iter__(self) -> OpaqueThinIter: ...

class OptionEnum:
    def __init__(self, arg: OptionEnum.OptionEnum, /) -> None: ...

    class OptionEnum(enum.Enum):
        Foo = 0

        Bar = 1

        Baz = 2

    Foo: OptionEnum.OptionEnum = OptionEnum.OptionEnum.Foo

    Bar: OptionEnum.OptionEnum = OptionEnum.OptionEnum.Bar

    Baz: OptionEnum.OptionEnum = OptionEnum.OptionEnum.Baz

    def __eq__(self, arg: OptionEnum.OptionEnum, /) -> bool: ...

    def __repr__(self) -> str: ...

class OptionInputStruct:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, a: int | None, b: str | None, c: OptionEnum | None) -> None: ...

    @property
    def a(self) -> int | None: ...

    @a.setter
    def a(self, arg: int, /) -> None: ...

    @property
    def b(self) -> str | None: ...

    @b.setter
    def b(self, arg: str, /) -> None: ...

    @property
    def c(self) -> OptionEnum | None: ...

    @c.setter
    def c(self, arg: OptionEnum, /) -> None: ...

class OptionOpaque:
    @staticmethod
    def accepts_borrowing_option_struct(arg: BorrowingOptionStruct) -> None: ...

    @staticmethod
    def accepts_multiple_option_enum(sentinel1: int, arg1: OptionEnum | None = None, arg2: OptionEnum | None = None, arg3: OptionEnum | None = None, sentinel2: int) -> OptionEnum | None: ...

    @staticmethod
    def accepts_option_enum(arg: OptionEnum | None = None, sentinel: int) -> OptionEnum | None: ...

    @staticmethod
    def accepts_option_input_struct(arg: OptionInputStruct | None = None, sentinel: int) -> OptionInputStruct | None: ...

    @staticmethod
    def accepts_option_primitive(arg: Sequence[int] | None = None, sentinel: int) -> int: ...

    @staticmethod
    def accepts_option_str(arg: str | None = None, sentinel: int) -> int: ...

    @staticmethod
    def accepts_option_str_slice(arg: Sequence[str] | None = None, sentinel: int) -> bool: ...

    @staticmethod
    def accepts_option_u8(arg: int | None = None, sentinel: int) -> int | None: ...

    def assert_integer(self, i: int) -> None: ...

    @staticmethod
    def new(i: int) -> OptionOpaque: ...

    @staticmethod
    def new_none() -> OptionOpaque: ...

    @staticmethod
    def new_struct() -> OptionStruct: ...

    @staticmethod
    def new_struct_nones() -> OptionStruct: ...

    def option_i32(self) -> int | None: ...

    def option_isize(self) -> int | None: ...

    @staticmethod
    def option_opaque_argument(arg: OptionOpaque | None = None) -> bool: ...

    def option_u32(self) -> int | None: ...

    def option_usize(self) -> int | None: ...

    @staticmethod
    def returns() -> OptionStruct | None: ...

    def returns_none_self(self) -> OptionOpaque: ...

    @staticmethod
    def returns_option_input_struct() -> OptionInputStruct: ...

    def returns_some_self(self) -> OptionOpaque: ...

class OptionOpaqueChar:
    def assert_char(self, ch: str) -> None: ...

class OptionString:
    def borrow(self) -> str | None: ...

    @staticmethod
    def new(diplomat_str: str) -> OptionString: ...

    def write(self) -> str: ...

class OptionStruct:
    @property
    def a(self) -> OptionOpaque: ...

    @property
    def b(self) -> OptionOpaqueChar: ...

    @property
    def c(self) -> int: ...

    @property
    def d(self) -> OptionOpaque: ...

class OutTupleStruct:
    @property
    def x(self) -> int: ...

    @property
    def y(self) -> int: ...

    @property
    def primitive(self) -> PrimitiveStruct: ...

    @property
    def opaque(self) -> Opaque: ...

    @staticmethod
    def new() -> tuple[int, int, PrimitiveStruct, Opaque]: ...

class PrimitiveStruct:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, x: float | None, a: bool | None, b: str | None, c: int | None, d: int | None, e: int | None) -> None: ...

    @property
    def x(self) -> float: ...

    @x.setter
    def x(self, arg: float, /) -> None: ...

    @property
    def a(self) -> bool: ...

    @a.setter
    def a(self, arg: bool, /) -> None: ...

    @property
    def b(self) -> str: ...

    @b.setter
    def b(self, arg: str, /) -> None: ...

    @property
    def c(self) -> int: ...

    @c.setter
    def c(self, arg: int, /) -> None: ...

    @property
    def d(self) -> int: ...

    @d.setter
    def d(self, arg: int, /) -> None: ...

    @property
    def e(self) -> int: ...

    @e.setter
    def e(self, arg: int, /) -> None: ...

    def mutable_ref(self, a: PrimitiveStruct) -> None: ...

class PrimitiveStructSlice:
    @overload
    def __init__(self) -> None:
        """Default constructor"""

    @overload
    def __init__(self, arg: PrimitiveStructSlice) -> None:
        """Copy constructor"""

    @overload
    def __init__(self, arg: Iterable[PrimitiveStruct], /) -> None:
        """Construct from an iterable object"""

    def __len__(self) -> int: ...

    def __bool__(self) -> bool:
        """Check whether the vector is nonempty"""

    def __repr__(self) -> str: ...

    def __iter__(self) -> Iterator[PrimitiveStruct]: ...

    @overload
    def __getitem__(self, arg: int, /) -> PrimitiveStruct: ...

    @overload
    def __getitem__(self, arg: slice, /) -> PrimitiveStructSlice: ...

    def clear(self) -> None:
        """Remove all items from list."""

    def append(self, arg: PrimitiveStruct, /) -> None:
        """Append `arg` to the end of the list."""

    def insert(self, arg0: int, arg1: PrimitiveStruct, /) -> None:
        """Insert object `arg1` before index `arg0`."""

    def pop(self, index: int = -1) -> PrimitiveStruct:
        """Remove and return item at `index` (default last)."""

    def extend(self, arg: PrimitiveStructSlice, /) -> None:
        """Extend `self` by appending elements from `arg`."""

    @overload
    def __setitem__(self, arg0: int, arg1: PrimitiveStruct, /) -> None: ...

    @overload
    def __setitem__(self, arg0: slice, arg1: PrimitiveStructSlice, /) -> None: ...

    @overload
    def __delitem__(self, arg: int, /) -> None: ...

    @overload
    def __delitem__(self, arg: slice, /) -> None: ...

class PrimitiveStructVec:
    def __init__(self) -> None: ...

    def __len__(self) -> int: ...

    def append(self, value: PrimitiveStruct) -> None: ...

    @property
    def asSlice(self) -> list[PrimitiveStruct]: ...

    def __getitem__(self, idx: int) -> PrimitiveStruct | None: ...

    @staticmethod
    def take_in_slice(a: Sequence[PrimitiveStruct]) -> PrimitiveStructVec: ...

    @staticmethod
    def take_slice_from_other_namespace(_sl: Sequence[ns.RenamedStructWithAttrs]) -> None: ...

class RefList:
    @staticmethod
    def node(data: RefListParameter) -> RefList: ...

class RefListParameter:
    pass

class ResultOpaque:
    def __init__(self, i: int) -> None: ...

    def assert_integer(self, i: int) -> None: ...

    def give_self(self) -> None: ...

    @staticmethod
    def new_failing_bar() -> ResultOpaque: ...

    @staticmethod
    def new_failing_foo() -> ResultOpaque: ...

    @staticmethod
    def new_failing_struct(i: int) -> ResultOpaque: ...

    @staticmethod
    def new_failing_unit() -> ResultOpaque: ...

    @staticmethod
    def new_in_enum_err(i: int) -> ErrorEnum: ...

    @staticmethod
    def new_in_err(i: int) -> None: ...

    @staticmethod
    def new_int(i: int) -> int: ...

    def __str__(self) -> str: ...

    def takes_str(self, _v: str) -> ResultOpaque:
        """
        When we take &str, the return type becomes a Result
        Test that this interacts gracefully with returning a reference type
        """

class ScalarPairWithPadding:
    """Testing JS-specific layout/padding behavior"""

    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, first: int | None, second: int | None) -> None: ...

    @property
    def first(self) -> int: ...

    @first.setter
    def first(self, arg: int, /) -> None: ...

    @property
    def second(self) -> int: ...

    @second.setter
    def second(self, arg: int, /) -> None: ...

    def assert_value(self) -> None: ...

class ScalarPairWithPaddingSlice:
    @overload
    def __init__(self) -> None:
        """Default constructor"""

    @overload
    def __init__(self, arg: ScalarPairWithPaddingSlice) -> None:
        """Copy constructor"""

    @overload
    def __init__(self, arg: Iterable[ScalarPairWithPadding], /) -> None:
        """Construct from an iterable object"""

    def __len__(self) -> int: ...

    def __bool__(self) -> bool:
        """Check whether the vector is nonempty"""

    def __repr__(self) -> str: ...

    def __iter__(self) -> Iterator[ScalarPairWithPadding]: ...

    @overload
    def __getitem__(self, arg: int, /) -> ScalarPairWithPadding: ...

    @overload
    def __getitem__(self, arg: slice, /) -> ScalarPairWithPaddingSlice: ...

    def clear(self) -> None:
        """Remove all items from list."""

    def append(self, arg: ScalarPairWithPadding, /) -> None:
        """Append `arg` to the end of the list."""

    def insert(self, arg0: int, arg1: ScalarPairWithPadding, /) -> None:
        """Insert object `arg1` before index `arg0`."""

    def pop(self, index: int = -1) -> ScalarPairWithPadding:
        """Remove and return item at `index` (default last)."""

    def extend(self, arg: ScalarPairWithPaddingSlice, /) -> None:
        """Extend `self` by appending elements from `arg`."""

    @overload
    def __setitem__(self, arg0: int, arg1: ScalarPairWithPadding, /) -> None: ...

    @overload
    def __setitem__(self, arg0: slice, arg1: ScalarPairWithPaddingSlice, /) -> None: ...

    @overload
    def __delitem__(self, arg: int, /) -> None: ...

    @overload
    def __delitem__(self, arg: slice, /) -> None: ...

class StructArithmetic:
    def __init__(self, x: int, y: int) -> None: ...

    @property
    def x(self) -> int: ...

    @x.setter
    def x(self, arg: int, /) -> None: ...

    @property
    def y(self) -> int: ...

    @y.setter
    def y(self, arg: int, /) -> None: ...

    ORIGIN: somelib.StructArithmetic = ...
    """(arg: object, /) -> somelib.StructArithmetic"""

    def __add__(self, arg: StructArithmetic, /) -> StructArithmetic: ...

    def __truediv__(self, arg: StructArithmetic, /) -> StructArithmetic: ...

    def __mul__(self, arg: StructArithmetic, /) -> StructArithmetic: ...

    def __sub__(self, arg: StructArithmetic, /) -> StructArithmetic: ...

class StructOfOpaque:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, i: Opaque | None, j: OpaqueMut | None) -> None: ...

    @property
    def i(self) -> Opaque: ...

    @i.setter
    def i(self, arg: Opaque, /) -> None: ...

    @property
    def j(self) -> OpaqueMut: ...

    @j.setter
    def j(self, arg: OpaqueMut, /) -> None: ...

    def take_in(self, other: Opaque) -> None: ...

class StructWithSlices:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, first: str | None, second: Sequence[int] | None) -> None: ...

    @property
    def first(self) -> str: ...

    @first.setter
    def first(self, arg: str, /) -> None: ...

    @property
    def second(self) -> list[int]: ...

    @second.setter
    def second(self, arg: Sequence[int], /) -> None: ...

    def return_last(self) -> str: ...

class TupleStruct:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, x: int | None, y: int | None, st: MyStruct | None, op: Opaque | None) -> None: ...

    @property
    def x(self) -> int: ...

    @x.setter
    def x(self, arg: int, /) -> None: ...

    @property
    def y(self) -> int: ...

    @y.setter
    def y(self, arg: int, /) -> None: ...

    @property
    def st(self) -> MyStruct: ...

    @st.setter
    def st(self, arg: MyStruct, /) -> None: ...

    @property
    def op(self) -> Opaque: ...

    @op.setter
    def op(self, arg: Opaque, /) -> None: ...

    @staticmethod
    def takes_containing(c: tuple[tuple[int, int, MyStruct, Opaque]]) -> str: ...

    @overload
    @staticmethod
    def takes_st_as_tuple(a: tuple[int, int, MyStruct, Opaque]) -> int: ...

    @overload
    @staticmethod
    def takes_st_as_tuple(a: tuple[int, int, MyStruct, Opaque], i: int) -> int: ...

class Two:
    pass

class UnimportedEnum:
    def __init__(self, arg: UnimportedEnum.UnimportedEnum, /) -> None: ...

    class UnimportedEnum(enum.Enum):
        A = 0

        B = 1

        C = 2

    A: UnimportedEnum.UnimportedEnum = UnimportedEnum.UnimportedEnum.A

    B: UnimportedEnum.UnimportedEnum = UnimportedEnum.UnimportedEnum.B

    C: UnimportedEnum.UnimportedEnum = UnimportedEnum.UnimportedEnum.C

    def __eq__(self, arg: UnimportedEnum.UnimportedEnum, /) -> bool: ...

    def __repr__(self) -> str: ...

class Unnamespaced:
    @staticmethod
    def make(_e: ns.RenamedAttrEnum) -> Unnamespaced: ...

    def use_namespaced(self, _n: ns.AttrOpaque1Renamed) -> None: ...

class Utf16Wrap:
    def __init__(self, input: "std::basic_string_view<char16_t, std::char_traits<char16_t> >") -> None: ...

    def borrow_cont(self) -> "std::basic_string_view<char16_t, std::char_traits<char16_t> >": ...

    def get_debug_str(self) -> str: ...

class __dummy__:
    pass

def free_callback_holder(f: Callable[[], None]) -> None: ...

class monostate:
    def __repr__(self) -> str: ...

    def __str__(self) -> str: ...
