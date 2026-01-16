#include "diplomat_nanobind_common.hpp"


#include "CyclicStructB.hpp"
NB_MAKE_OPAQUE(std::vector<somelib::CyclicStructB>)

namespace somelib {
void add_CyclicStructB_binding(nb::module_ mod) {
    
    // Python lists are represented as PyObject**, which runs somewhat counter to any use cases where we want to be able to transparently pass over lists without copying over memory in any ways.
    // bind_vector solves this issue by exposing std::vector<somelib::CyclicStructB> as a type that will exist inside of C++, with functions to access its memory from Python.
    // TL;DR: this creates a faux list type that makes it easier to pass vectors of this type in Python without copying. 
    nb::bind_vector<std::vector<somelib::CyclicStructB>>(mod, "CyclicStructBSlice"); 
    nb::class_<somelib::CyclicStructB> st(mod, "CyclicStructB");
    st
        .def(nb::init<>())
        .def(nb::init<uint8_t>(), "field"_a.none())
        .def_rw("field", &somelib::CyclicStructB::field)
        .def_static("get_a", &somelib::CyclicStructB::get_a)
        .def_static("get_a_option", &somelib::CyclicStructB::get_a_option);
}

} 