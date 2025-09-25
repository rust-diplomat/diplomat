#include "diplomat_nanobind_common.hpp"


#include "DefaultEnum.hpp"
#include "MyStruct.hpp"
#include "MyStructContainingAnOption.hpp"


void add_MyStructContainingAnOption_binding(nb::module_ mod) {
    nb::class_<MyStructContainingAnOption>(mod, "MyStructContainingAnOption")
        .def_rw("a", &MyStructContainingAnOption::a)
        .def_rw("b", &MyStructContainingAnOption::b)
        .def_static("filled", &MyStructContainingAnOption::filled)
        .def("__init__", [](MyStructContainingAnOption* self){ *self = MyStructContainingAnOption::new_(); });
}

