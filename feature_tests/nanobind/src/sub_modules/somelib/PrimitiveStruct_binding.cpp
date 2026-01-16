#include "diplomat_nanobind_common.hpp"


#include "PrimitiveStruct.hpp"
NB_MAKE_OPAQUE(std::vector<somelib::PrimitiveStruct>)

namespace somelib {
void add_PrimitiveStruct_binding(nb::module_ mod) {
    
    // Python lists are represented as PyObject**, which runs somewhat counter to any use cases where we want to be able to transparently pass over lists without copying over memory in any ways.
    // bind_vector solves this issue by exposing std::vector<somelib::PrimitiveStruct> as a type that will exist inside of C++, with functions to access its memory from Python.
    // TL;DR: this creates a faux list type that makes it easier to pass vectors of this type in Python without copying. 
    nb::bind_vector<std::vector<somelib::PrimitiveStruct>>(mod, "PrimitiveStructSlice"); 
    nb::class_<somelib::PrimitiveStruct> st(mod, "PrimitiveStruct");
    st
        .def(nb::init<>())
        .def(nb::init<float, bool, char32_t, int64_t, intptr_t, uint8_t>(), "x"_a.none(),  "a"_a.none(),  "b"_a.none(),  "c"_a.none(),  "d"_a.none(),  "e"_a.none())
        .def_rw("x", &somelib::PrimitiveStruct::x)
        .def_rw("a", &somelib::PrimitiveStruct::a)
        .def_rw("b", &somelib::PrimitiveStruct::b)
        .def_rw("c", &somelib::PrimitiveStruct::c)
        .def_rw("d", &somelib::PrimitiveStruct::d)
        .def_rw("e", &somelib::PrimitiveStruct::e)
        .def("mutable_ref", &somelib::PrimitiveStruct::mutable_ref, "a"_a)
        .def_static("mutable_slice", &somelib::PrimitiveStruct::mutable_slice, "a"_a);
}

} 