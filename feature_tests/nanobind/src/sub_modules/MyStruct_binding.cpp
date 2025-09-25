#include "diplomat_nanobind_common.hpp"


#include "MyEnum.hpp"
#include "MyStruct.hpp"


void add_MyStruct_binding(nb::module_ mod) {
    nb::class_<MyStruct>(mod, "MyStruct")
        .def_rw("a", &MyStruct::a)
        .def_rw("b", &MyStruct::b)
        .def_rw("c", &MyStruct::c)
        .def_rw("d", &MyStruct::d)
        .def_rw("e", &MyStruct::e)
        .def_rw("f", &MyStruct::f)
        .def_rw("g", &MyStruct::g)
        .def_static("fails_zst_result", &MyStruct::fails_zst_result)
        .def("into_a", &MyStruct::into_a)
        .def("__init__", [](MyStruct* self){ *self = MyStruct::new_(); })
        .def_static("returns_zst_result", &MyStruct::returns_zst_result)
        .def("takes_const", &MyStruct::takes_const, "o"_a)
        .def("takes_mut", &MyStruct::takes_mut, "o"_a);
}

