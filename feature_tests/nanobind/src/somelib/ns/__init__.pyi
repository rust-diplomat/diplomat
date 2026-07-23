from collections.abc import Callable, Iterable, Iterator, Sequence
import enum
from typing import overload

import somelib


class AttrOpaque1Renamed:
    """
    Some example docs
    Some Nanobind/C++ example docs
    Back to all docs
    """

    @overload
    def __init__(self) -> None:
        """More example docs"""

    @overload
    def __init__(self, _i: int) -> None: ...

    @property
    def abirenamed(self) -> int: ...

    @staticmethod
    def hello() -> int: ...

    @staticmethod
    def mac_test() -> int: ...

    @property
    def method(self) -> int: ...

    @staticmethod
    def test_namespaced_callback(_t: Callable[[], None]) -> None: ...

    def use_namespaced(self, _n: RenamedAttrEnum) -> None: ...

    def use_unnamespaced(self, _un: somelib.Unnamespaced) -> None: ...

class RenamedAttrEnum:
    def __init__(self, arg: RenamedAttrEnum.RenamedAttrEnum, /) -> None: ...

    class RenamedAttrEnum(enum.Enum):
        A = 0

        B = 1

        Renamed = 2

    A: RenamedAttrEnum.RenamedAttrEnum = RenamedAttrEnum.RenamedAttrEnum.A

    B: RenamedAttrEnum.RenamedAttrEnum = RenamedAttrEnum.RenamedAttrEnum.B

    Renamed: RenamedAttrEnum.RenamedAttrEnum = RenamedAttrEnum.RenamedAttrEnum.Renamed

    def __eq__(self, arg: RenamedAttrEnum.RenamedAttrEnum, /) -> bool: ...

    def __repr__(self) -> str: ...

class RenamedAttrOpaque2:
    pass

class RenamedBlockOverride:
    def special_function() -> str: ...

class RenamedComparable:
    def __eq__(self, arg: RenamedComparable, /) -> bool: ...

    def __ne__(self, arg: RenamedComparable, /) -> bool: ...

    def __le__(self, arg: RenamedComparable, /) -> bool: ...

    def __ge__(self, arg: RenamedComparable, /) -> bool: ...

    def __lt__(self, arg: RenamedComparable, /) -> bool: ...

    def __gt__(self, arg: RenamedComparable, /) -> bool: ...

    @staticmethod
    def new(int: int) -> RenamedComparable: ...

class RenamedDeprecatedEnum:
    """.. deprecated:: use Foo"""

    def __init__(self, arg: RenamedDeprecatedEnum.RenamedDeprecatedEnum, /) -> None: ...

    class RenamedDeprecatedEnum(enum.Enum):
        A = 0

    A: RenamedDeprecatedEnum.RenamedDeprecatedEnum = RenamedDeprecatedEnum.RenamedDeprecatedEnum.A

    def __eq__(self, arg: RenamedDeprecatedEnum.RenamedDeprecatedEnum, /) -> bool: ...

    def __repr__(self) -> str: ...

class RenamedDeprecatedOpaque:
    """.. deprecated:: use Foo"""

class RenamedDeprecatedStruct:
    """.. deprecated:: use Foo"""

    def __init__(self) -> None: ...

class RenamedMixinTest:
    @staticmethod
    def hello() -> str: ...

class RenamedMyIndexer:
    def __init__(self, v: Sequence[str]) -> None: ...

    @overload
    def __getitem__(self, i: int) -> str | None: ...

    @overload
    def __getitem__(self, s: str) -> str | None: ...

class RenamedMyIterable:
    def __init__(self, x: Sequence[int]) -> None: ...

    def __len__(self) -> int: ...

    def __iter__(self) -> RenamedMyIterator: ...

class RenamedMyIterator:
    def __next__(self) -> int | None: ...

    def __iter__(self) -> object: ...

class RenamedOpaqueArithmetic:
    def __add__(self, arg: RenamedOpaqueArithmetic, /) -> RenamedOpaqueArithmetic: ...

    def __iadd__(self, arg: RenamedOpaqueArithmetic, /) -> None: ...

    def __truediv__(self, arg: RenamedOpaqueArithmetic, /) -> RenamedOpaqueArithmetic: ...

    def __itruediv__(self, arg: RenamedOpaqueArithmetic, /) -> None: ...

    @overload
    @staticmethod
    def make(x: int, y: int = 12) -> RenamedOpaqueArithmetic: ...

    @overload
    @staticmethod
    def make(x: float, y: float = 14.48, z: float | None = 0) -> RenamedOpaqueArithmetic: ...

    @overload
    @staticmethod
    def make(x: float, z: bool) -> RenamedOpaqueArithmetic: ...

    def __mul__(self, arg: RenamedOpaqueArithmetic, /) -> RenamedOpaqueArithmetic: ...

    def __imul__(self, arg: RenamedOpaqueArithmetic, /) -> None: ...

    def __sub__(self, arg: RenamedOpaqueArithmetic, /) -> RenamedOpaqueArithmetic: ...

    def __isub__(self, arg: RenamedOpaqueArithmetic, /) -> None: ...

    @overload
    def x(self) -> int: ...

    @overload
    def x(self, add: int) -> int: ...

    def y(self) -> int: ...

class RenamedOpaqueIterable:
    def __init__(self, size: int) -> None: ...

    def __iter__(self) -> RenamedOpaqueIterator: ...

class RenamedOpaqueIterator:
    def __next__(self) -> AttrOpaque1Renamed | None: ...

    def __iter__(self) -> object: ...

class RenamedOpaqueRefIterable:
    def __init__(self, size: int) -> None: ...

    def __iter__(self) -> RenamedOpaqueRefIterator: ...

class RenamedOpaqueRefIterator:
    def __next__(self) -> AttrOpaque1Renamed | None: ...

    def __iter__(self) -> object: ...

class RenamedOpaqueZST:
    """
    Tests for https://github.com/rust-diplomat/diplomat/issues/1050.
    C++ generates unique_ptrs for Opaque ZSTs, and Nanobind
    expects every unique_ptr it converts to wrap a unique pointer type. It errors otherwise.
    This is not the case, as in Rust pointers to ZSTs are always the same address.
    """

    def __init__(self) -> None: ...

    def __add__(self, arg: RenamedOpaqueZST, /) -> RenamedOpaqueZST: ...

    def __truediv__(self, arg: RenamedOpaqueZST, /) -> RenamedOpaqueZST: ...

    @staticmethod
    def fail_zst(return_success: bool) -> None: ...

    @property
    def getter(self) -> RenamedOpaqueZST: ...

    @getter.setter
    def getter(self, arg: RenamedOpaqueZST, /) -> None: ...

    def __getitem__(self, _idx: int) -> RenamedOpaqueZST | None: ...

    def __iter__(self) -> RenamedOpaqueZSTIterator: ...

    @staticmethod
    def make() -> RenamedOpaqueZST: ...

    def member(self) -> RenamedOpaqueZST: ...

    def __mul__(self, arg: RenamedOpaqueZST, /) -> RenamedOpaqueZST: ...

    def mut_member(self) -> RenamedOpaqueZST: ...

    @staticmethod
    def optional_zst(is_some: bool) -> RenamedOpaqueZST | None: ...

    out_string: str = ...
    """(arg: object, /) -> str"""

    static_getter: somelib.ns.RenamedOpaqueZST = ...
    """(arg: object, /) -> somelib.ns.RenamedOpaqueZST"""

    def __sub__(self, arg: RenamedOpaqueZST, /) -> RenamedOpaqueZST: ...

    @staticmethod
    def success_fail_zst(return_success: bool) -> RenamedOpaqueZST: ...

    @staticmethod
    def success_zst(return_success: bool) -> RenamedOpaqueZST: ...

class RenamedOpaqueZSTIndexer:
    def __init__(self) -> None: ...

    def __getitem__(self, idx: int) -> RenamedOpaqueZSTIndexer | None: ...

class RenamedOpaqueZSTIterator:
    """Tests for https://github.com/rust-diplomat/diplomat/issues/1050."""

    def __init__(self) -> None: ...

    def __next__(self) -> RenamedOpaqueZSTIterator | None: ...

    def __iter__(self) -> object: ...

    def __getitem__(self, _idx: int) -> RenamedOpaqueZSTIterator | None: ...

    def __str__(self) -> str: ...

class RenamedPartialComparable:
    def __init__(self, float_: float) -> None: ...

    def __eq__(self, arg: RenamedPartialComparable, /) -> bool | None: ...

    def __ne__(self, arg: RenamedPartialComparable, /) -> bool | None: ...

    def __le__(self, arg: RenamedPartialComparable, /) -> bool | None: ...

    def __ge__(self, arg: RenamedPartialComparable, /) -> bool | None: ...

    def __lt__(self, arg: RenamedPartialComparable, /) -> bool | None: ...

    def __gt__(self, arg: RenamedPartialComparable, /) -> bool | None: ...

    def test_nonstd(self, other: RenamedPartialComparable) -> int | None: ...

class RenamedPartialComparableSlice:
    @overload
    def __init__(self) -> None: ...

    @overload
    def __init__(self, f: float | None) -> None: ...

    @property
    def f(self) -> float: ...

    @f.setter
    def f(self, arg: float, /) -> None: ...

    def __eq__(self, arg: RenamedPartialComparableSlice, /) -> bool | None: ...

    def __ne__(self, arg: RenamedPartialComparableSlice, /) -> bool | None: ...

    def __le__(self, arg: RenamedPartialComparableSlice, /) -> bool | None: ...

    def __ge__(self, arg: RenamedPartialComparableSlice, /) -> bool | None: ...

    def __lt__(self, arg: RenamedPartialComparableSlice, /) -> bool | None: ...

    def __gt__(self, arg: RenamedPartialComparableSlice, /) -> bool | None: ...

class RenamedPartialComparableSliceSlice:
    @overload
    def __init__(self) -> None:
        """Default constructor"""

    @overload
    def __init__(self, arg: RenamedPartialComparableSliceSlice) -> None:
        """Copy constructor"""

    @overload
    def __init__(self, arg: Iterable[RenamedPartialComparableSlice], /) -> None:
        """Construct from an iterable object"""

    def __len__(self) -> int: ...

    def __bool__(self) -> bool:
        """Check whether the vector is nonempty"""

    def __repr__(self) -> str: ...

    def __iter__(self) -> Iterator[RenamedPartialComparableSlice]: ...

    @overload
    def __getitem__(self, arg: int, /) -> RenamedPartialComparableSlice: ...

    @overload
    def __getitem__(self, arg: slice, /) -> RenamedPartialComparableSliceSlice: ...

    def clear(self) -> None:
        """Remove all items from list."""

    def append(self, arg: RenamedPartialComparableSlice, /) -> None:
        """Append `arg` to the end of the list."""

    def insert(self, arg0: int, arg1: RenamedPartialComparableSlice, /) -> None:
        """Insert object `arg1` before index `arg0`."""

    def pop(self, index: int = -1) -> RenamedPartialComparableSlice:
        """Remove and return item at `index` (default last)."""

    def extend(self, arg: RenamedPartialComparableSliceSlice, /) -> None:
        """Extend `self` by appending elements from `arg`."""

    @overload
    def __setitem__(self, arg0: int, arg1: RenamedPartialComparableSlice, /) -> None: ...

    @overload
    def __setitem__(self, arg0: slice, arg1: RenamedPartialComparableSliceSlice, /) -> None: ...

    @overload
    def __delitem__(self, arg: int, /) -> None: ...

    @overload
    def __delitem__(self, arg: slice, /) -> None: ...

class RenamedRenamedCachedIncludeZST:
    def __init__(self) -> None: ...

class RenamedStringList:
    """Testing support for List[str] in Nanobind"""

    @staticmethod
    def return_new() -> list[str]: ...

class RenamedStructWithAttrs:
    def __init__(self, a: bool, b: int) -> None: ...

    @property
    def a(self) -> bool: ...

    @a.setter
    def a(self, arg: bool, /) -> None: ...

    @property
    def b(self) -> int: ...

    @b.setter
    def b(self, arg: int, /) -> None: ...

    @property
    def c(self) -> int: ...

    def deprecated(self) -> None:
        """.. deprecated:: use Foo"""

class RenamedStructWithAttrsSlice:
    @overload
    def __init__(self) -> None:
        """Default constructor"""

    @overload
    def __init__(self, arg: RenamedStructWithAttrsSlice) -> None:
        """Copy constructor"""

    @overload
    def __init__(self, arg: Iterable[RenamedStructWithAttrs], /) -> None:
        """Construct from an iterable object"""

    def __len__(self) -> int: ...

    def __bool__(self) -> bool:
        """Check whether the vector is nonempty"""

    def __repr__(self) -> str: ...

    def __iter__(self) -> Iterator[RenamedStructWithAttrs]: ...

    @overload
    def __getitem__(self, arg: int, /) -> RenamedStructWithAttrs: ...

    @overload
    def __getitem__(self, arg: slice, /) -> RenamedStructWithAttrsSlice: ...

    def clear(self) -> None:
        """Remove all items from list."""

    def append(self, arg: RenamedStructWithAttrs, /) -> None:
        """Append `arg` to the end of the list."""

    def insert(self, arg0: int, arg1: RenamedStructWithAttrs, /) -> None:
        """Insert object `arg1` before index `arg0`."""

    def pop(self, index: int = -1) -> RenamedStructWithAttrs:
        """Remove and return item at `index` (default last)."""

    def extend(self, arg: RenamedStructWithAttrsSlice, /) -> None:
        """Extend `self` by appending elements from `arg`."""

    @overload
    def __setitem__(self, arg0: int, arg1: RenamedStructWithAttrs, /) -> None: ...

    @overload
    def __setitem__(self, arg0: slice, arg1: RenamedStructWithAttrsSlice, /) -> None: ...

    @overload
    def __delitem__(self, arg: int, /) -> None: ...

    @overload
    def __delitem__(self, arg: slice, /) -> None: ...

class RenamedTestMacroStruct:
    def __init__(self) -> None: ...

    @property
    def a(self) -> int: ...

    @a.setter
    def a(self, arg: int, /) -> None: ...

    @staticmethod
    def test_func() -> int: ...

class RenamedTestOpaque:
    pass

class RenamedVectorTest:
    def __init__(self) -> None: ...

    def __getitem__(self, idx: int) -> float | None: ...

    @property
    def len(self) -> int: ...

    def push(self, value: float) -> None: ...

def Renamedfree_func_test(x: int) -> int: ...
