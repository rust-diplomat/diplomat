#include "diplomat_nanobind_common.hpp"


#include "ContiguousEnum.hpp"
#include "MyEnum.hpp"
#include "MyStruct.hpp"
NB_MAKE_OPAQUE(std::vector<somelib::MyStruct>)

namespace somelib {
void add_MyStruct_binding(nb::module_ mod) {
    
    // Python lists are represented as PyObject**, which runs somewhat counter to any use cases where we want to be able to transparently pass over lists without copying over memory in any ways.
    // bind_vector solves this issue by exposing std::vector<somelib::MyStruct> as a type that will exist inside of C++, with functions to access its memory from Python.
    // TL;DR: this creates a faux list type that makes it easier to pass vectors of this type in Python without copying. 
    nb::bind_vector<std::vector<somelib::MyStruct>>(mod, "MyStructSlice"); 
    nb::class_<somelib::MyStruct> st(mod, "MyStruct");
    st
        .def_rw("a", &somelib::MyStruct::a)
        .def_rw("b", &somelib::MyStruct::b)
        .def_rw("c", &somelib::MyStruct::c)
        .def_rw("d", &somelib::MyStruct::d)
        .def_rw("e", &somelib::MyStruct::e)
        .def_rw("f", &somelib::MyStruct::f)
        .def_rw("g", &somelib::MyStruct::g)
        .def(nb::new_(&somelib::MyStruct::new_))
        .def(nb::new_(&somelib::MyStruct::new_overload), "i"_a)
        .def_static("assert_enum_slice", &somelib::MyStruct::assert_enum_slice, "slice"_a, "second_value"_a)
        .def_static("assert_slice", &somelib::MyStruct::assert_slice, "slice"_a, "second_value"_a)
        .def_static("fails_zst_result", &somelib::MyStruct::fails_zst_result)
        .def("into_a", &somelib::MyStruct::into_a)
        .def_static("returns_zst_result", &somelib::MyStruct::returns_zst_result)
        .def("take_ref_ret", &somelib::MyStruct::take_ref_ret)
        .def("takes_const", &somelib::MyStruct::takes_const, "o"_a)
        .def("takes_mut", &somelib::MyStruct::takes_mut, "o"_a);
}

} 