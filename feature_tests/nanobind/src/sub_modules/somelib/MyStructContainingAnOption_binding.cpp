#include "diplomat_nanobind_common.hpp"


#include "DefaultEnum.hpp"
#include "MyStruct.hpp"
#include "MyStructContainingAnOption.hpp"

namespace somelib {
void add_MyStructContainingAnOption_binding(nb::module_ mod) {
    nb::class_<somelib::MyStructContainingAnOption> st(mod, "MyStructContainingAnOption");
    st
        .def_rw("a", &somelib::MyStructContainingAnOption::a)
        .def_rw("b", &somelib::MyStructContainingAnOption::b)
        .def_static("filled", &somelib::MyStructContainingAnOption::filled)
        .def("__init__", [](somelib::MyStructContainingAnOption* self){ *self = somelib::MyStructContainingAnOption::new_(); });
}

} 