#include "diplomat_nanobind_common.hpp"


#include "BigStructWithStuff.hpp"
#include "ScalarPairWithPadding.hpp"
NB_MAKE_OPAQUE(std::vector<BigStructWithStuff>)


void add_BigStructWithStuff_binding(nb::module_ mod) {
    
    // Python lists are represented as PyObject**, which runs somewhat counter to any use cases where we want to be able to transparently pass over lists without copying over memory in any ways.
    // bind_vector solves this issue by exposing std::vector<BigStructWithStuff> as a type that will exist inside of C++, with functions to access its memory from Python.
    // TL;DR: this creates a faux list type that makes it easier to pass vectors of this type in Python without copying. 
    nb::bind_vector<std::vector<BigStructWithStuff>>(mod, "BigStructWithStuffSlice"); 
    nb::class_<BigStructWithStuff>(mod, "BigStructWithStuff")
        .def(nb::init<>())
        .def(nb::init<uint8_t, uint16_t, uint16_t, ScalarPairWithPadding, uint8_t>(), "first"_a.none(),  "second"_a.none(),  "third"_a.none(),  "fourth"_a.none(),  "fifth"_a.none())
        .def_rw("first", &BigStructWithStuff::first)
        .def_rw("second", &BigStructWithStuff::second)
        .def_rw("third", &BigStructWithStuff::third)
        .def_rw("fourth", &BigStructWithStuff::fourth)
        .def_rw("fifth", &BigStructWithStuff::fifth)
        .def_static("assert_slice", &BigStructWithStuff::assert_slice, "slice"_a, "second_value"_a)
        .def("assert_value", &BigStructWithStuff::assert_value, "extra_val"_a);
}

