#include "diplomat_nanobind_common.hpp"


#include "CyclicStructA.hpp"
#include "CyclicStructB.hpp"
NB_MAKE_OPAQUE(std::vector<CyclicStructA>)


void add_CyclicStructA_binding(nb::module_ mod) {
    
    // Python lists are represented as PyObject**, which runs somewhat counter to any use cases where we want to be able to transparently pass over lists without copying over memory in any ways.
    // bind_vector solves this issue by exposing std::vector<CyclicStructA> as a type that will exist inside of C++, with functions to access its memory from Python.
    // TL;DR: this creates a faux list type that makes it easier to pass vectors of this type in Python without copying. 
    nb::bind_vector<std::vector<CyclicStructA>>(mod, "CyclicStructASlice"); 
    nb::class_<CyclicStructA>(mod, "CyclicStructA")
        .def(nb::init<>())
        .def(nb::init<CyclicStructB>(), "a"_a.none())
        .def_rw("a", &CyclicStructA::a)
        .def("cyclic_out", &CyclicStructA::cyclic_out)
        .def("double_cyclic_out", &CyclicStructA::double_cyclic_out, "cyclic_struct_a"_a)
        .def_static("get_b", &CyclicStructA::get_b)
        .def_prop_ro("getter_out", &CyclicStructA::getter_out)
        .def_static("nested_slice", &CyclicStructA::nested_slice, "sl"_a);
}

