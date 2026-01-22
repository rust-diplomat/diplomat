#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedStructWithAttrs.hpp"
NB_MAKE_OPAQUE(std::vector<somelib::ns::RenamedStructWithAttrs>)

namespace somelib::ns {
void add_RenamedStructWithAttrs_binding(nb::module_ mod) {
    
    // Python lists are represented as PyObject**, which runs somewhat counter to any use cases where we want to be able to transparently pass over lists without copying over memory in any ways.
    // bind_vector solves this issue by exposing std::vector<somelib::ns::RenamedStructWithAttrs> as a type that will exist inside of C++, with functions to access its memory from Python.
    // TL;DR: this creates a faux list type that makes it easier to pass vectors of this type in Python without copying. 
    nb::bind_vector<std::vector<somelib::ns::RenamedStructWithAttrs>>(mod, "RenamedStructWithAttrsSlice"); 
    nb::class_<somelib::ns::RenamedStructWithAttrs> st(mod, "RenamedStructWithAttrs");
    st
        .def_rw("a", &somelib::ns::RenamedStructWithAttrs::a)
        .def_rw("b", &somelib::ns::RenamedStructWithAttrs::b)
        .def_prop_ro("c", &somelib::ns::RenamedStructWithAttrs::c)
        .def("deprecated", &somelib::ns::RenamedStructWithAttrs::deprecated)
        .def("__init__", [](somelib::ns::RenamedStructWithAttrs* self, bool a, uint32_t b){ auto tmp = somelib::ns::RenamedStructWithAttrs::new_fallible(a, b);
                    if(tmp.is_ok()) {
                        *self = std::move(tmp).ok().value();
                    } else {
                        nb::cast(tmp); // This will raise a python error with the contents of the error type
                    }}, "a"_a, "b"_a);
}

} 