#include "diplomat_nanobind_common.hpp"


#include "BigStructWithStuff.hpp"
#include "ScalarPairWithPadding.hpp"
NB_MAKE_OPAQUE(std::vector<somelib::BigStructWithStuff>)

namespace somelib {
void add_BigStructWithStuff_binding(nb::module_ mod) {
    
    // Python lists are represented as PyObject**, which runs somewhat counter to any use cases where we want to be able to transparently pass over lists without copying over memory in any ways.
    // bind_vector solves this issue by exposing std::vector<somelib::BigStructWithStuff> as a type that will exist inside of C++, with functions to access its memory from Python.
    // TL;DR: this creates a faux list type that makes it easier to pass vectors of this type in Python without copying. 
    nb::bind_vector<std::vector<somelib::BigStructWithStuff>>(mod, "BigStructWithStuffSlice"); 
    nb::class_<somelib::BigStructWithStuff> st(mod, "BigStructWithStuff");
    st
        .def(nb::init<>())
        .def(nb::init<uint8_t, uint16_t, uint16_t, somelib::ScalarPairWithPadding, uint8_t>(), "first"_a.none(),  "second"_a.none(),  "third"_a.none(),  "fourth"_a.none(),  "fifth"_a.none())
        .def_rw("first", &somelib::BigStructWithStuff::first)
        .def_rw("second", &somelib::BigStructWithStuff::second)
        .def_rw("third", &somelib::BigStructWithStuff::third)
        .def_rw("fourth", &somelib::BigStructWithStuff::fourth)
        .def_rw("fifth", &somelib::BigStructWithStuff::fifth)
        .def_static("assert_slice", &somelib::BigStructWithStuff::assert_slice, "slice"_a, "second_value"_a)
        .def("assert_value", &somelib::BigStructWithStuff::assert_value, "extra_val"_a);
}

} 