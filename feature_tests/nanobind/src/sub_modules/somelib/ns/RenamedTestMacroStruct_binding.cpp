#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedTestMacroStruct.hpp"

namespace somelib::ns {
void add_RenamedTestMacroStruct_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedTestMacroStruct> st(mod, "RenamedTestMacroStruct");
    st
        .def_rw("a", &somelib::ns::RenamedTestMacroStruct::a)
        .def_static("test_func", &somelib::ns::RenamedTestMacroStruct::test_func)
        .def("__init__", [](somelib::ns::RenamedTestMacroStruct* self){ *self = somelib::ns::RenamedTestMacroStruct::test_meta(); });
}

} 