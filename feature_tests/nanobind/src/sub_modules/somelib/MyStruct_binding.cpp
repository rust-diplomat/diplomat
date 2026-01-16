#include "diplomat_nanobind_common.hpp"


#include "MyEnum.hpp"
#include "MyStruct.hpp"

namespace somelib {
void add_MyStruct_binding(nb::module_ mod) {
    nb::class_<somelib::MyStruct> st(mod, "MyStruct");
    st
        .def_rw("a", &somelib::MyStruct::a)
        .def_rw("b", &somelib::MyStruct::b)
        .def_rw("c", &somelib::MyStruct::c)
        .def_rw("d", &somelib::MyStruct::d)
        .def_rw("e", &somelib::MyStruct::e)
        .def_rw("f", &somelib::MyStruct::f)
        .def_rw("g", &somelib::MyStruct::g)
        .def_static("fails_zst_result", &somelib::MyStruct::fails_zst_result)
        .def("into_a", &somelib::MyStruct::into_a)
        .def("__init__", [](somelib::MyStruct* self){ *self = somelib::MyStruct::new_(); })
        .def_static("returns_zst_result", &somelib::MyStruct::returns_zst_result)
        .def("takes_const", &somelib::MyStruct::takes_const, "o"_a)
        .def("takes_mut", &somelib::MyStruct::takes_mut, "o"_a);
}

} 