#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedTestMacroStruct.hpp"


namespace ns{

void add_RenamedTestMacroStruct_binding(nb::module_ mod) {
    nb::class_<ns::RenamedTestMacroStruct>(mod, "RenamedTestMacroStruct")
        .def_rw("a", &ns::RenamedTestMacroStruct::a)
        .def_static("test_func", &ns::RenamedTestMacroStruct::test_func)
        .def("__init__", [](ns::RenamedTestMacroStruct* self){ *self = ns::RenamedTestMacroStruct::test_meta(); });
}


}
