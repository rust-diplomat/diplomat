#include "diplomat_nanobind_common.hpp"


#include "ScalarPairWithPadding.hpp"
NB_MAKE_OPAQUE(std::vector<ScalarPairWithPadding>)


void add_ScalarPairWithPadding_binding(nb::handle mod) {
    
    // Python lists are represented as PyObject**, which runs somewhat counter to any use cases where we want to be able to transparently pass over lists without copying over memory in any ways.
    // bind_vector solves this issue by exposing std::vector<ScalarPairWithPadding> as a type that will exist inside of C++, with functions to access its memory from Python.
    // TL;DR: this creates a faux list type that makes it easier to pass vectors of this type in Python without copying. 
    nb::bind_vector<std::vector<ScalarPairWithPadding>>(mod, "ScalarPairWithPaddingSlice"); 
    nb::class_<ScalarPairWithPadding>(mod, "ScalarPairWithPadding")
        .def(nb::init<>())
        .def(nb::init<uint8_t, uint32_t>(), "first"_a.none(),  "second"_a.none())
        .def_rw("first", &ScalarPairWithPadding::first)
        .def_rw("second", &ScalarPairWithPadding::second)
    	.def("assert_value", &ScalarPairWithPadding::assert_value);
}

